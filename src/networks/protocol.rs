use super::db::db;
use crate::blockchain::block::block_hash;
use crate::blockchain::block::Block;
use crate::blockchain::transactions;
use crate::blockchain::transactions::Transaction;
use crate::networks::events::BootHelper;
use crate::networks::events::MyBehaviour;
use crate::networks::events::NodeType;
use crate::networks::input;
use crate::networks::transport;
use crate::networks::validate;
use crate::networks::zgossipsub;
use crate::networks::zkademlia;
use crate::networks::zrequest;
use crate::networks::zrequest::BlockRequest;
use async_std::{io, task};
use async_trait::async_trait;
use components::struc::AccountInfo;
use components::struc::Accounts;
use components::struc::FriendsList;
use components::struc::NetworkEvent;
use components::struc::NetworkInfo;
use components::struc::Request;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use futures::{prelude::*, select};
use libp2p::gossipsub;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::identify::{Identify, IdentifyConfig};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::Kademlia;
use libp2p::request_response::{
    ProtocolName, ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
    RequestResponseEvent, RequestResponseMessage,
};
use libp2p::Multiaddr;
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig},
    swarm::SwarmEvent,
    PeerId, Swarm,
};
use std::cmp::{Eq, Ord, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::hash::Hash;
use std::iter;
use std::sync::Arc;

pub async fn into_protocol(
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    sender: Sender<NetworkInfo>,
    reciever: Receiver<Request>,
    remote_send: Sender<NetworkEvent>,
) -> Result<(), Box<dyn Error>> {
    let last_block: Block = db::try_get_last_block();
    let friends: FriendsList = db::try_get_friends();
    let accounts: Accounts = db::try_get_accounts();
    println!("{:?}", local_peer_id);
    //Initializing behaviors
    let mut swarm = {
        let transport = transport::build_transport(local_key.clone()).await?;
        let gossipsub: gossipsub::Gossipsub = zgossipsub::create_gossip(local_key.clone());
        let kademlia: Kademlia<MemoryStore> = zkademlia::create_kademlia(local_key.clone());
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let identify = Identify::new(IdentifyConfig::new(
            "1.0".to_string(),
            local_key.clone().public(),
        ));

        let request = RequestResponse::new(
            zrequest::BlockCodec(),
            iter::once((zrequest::RequestProtocol(), ProtocolSupport::Full)),
            Default::default(),
        );
        let behaviour = MyBehaviour {
            gossipsub,
            kademlia,
            identify,
            mdns,
            request,
            sender,
            remote_send,
            reciever,
            accounts,
            friends,
            mempool: transactions::MemPool::default(),
            last_block,
            boot_helper: BootHelper {
                temp_last: "".to_string(),
                old_last: "".to_string(),
                friends_last: Vec::new(),
                friends_last_block: Vec::new(),
                friends_accnts: Vec::new(),
            },
            node_type: NodeType::FullNode,
        };
        Swarm::new(transport, behaviour, local_peer_id)
    };
    //I'm not sure if there is an easier way to connect to friends that have dynamic IPs.
    //Thus each node will re-use their IPs. Probably a better way, but it's annoying

    //If i know my exisiting IPs, listen on them, or else create new ones.
    match db::get("addrs".to_string()) {
        Ok(None) => {
            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
            swarm.listen_on("/ip6/::0/tcp/0".parse()?)?;
            let mut my_addrs: Vec<Multiaddr> = Vec::new();
            for multi in swarm.listeners() {
                my_addrs.push(multi.clone());
            }
            db::put(
                db::serialize(&my_addrs).expect("Serialize error"),
                "addrs".to_string(),
            )
            .expect("deserialize error");
        }
        Ok(Some(vc)) => {
            if let Ok(my_addrs) = db::deserialize::<Vec<Multiaddr>>(&vc) {
                for multi in my_addrs {
                    swarm.listen_on(multi).expect("listen error");
                }
            }
        }
        _ => (),
    }
    //swarm.listen_on("/ip4/192.168.1.197/tcp/54005".parse()?)?;
    swarm = zkademlia::boot(swarm);
    let behaviour = swarm.behaviour_mut();
    for friends in behaviour.friends.list() {
        //Will either emit and error or the response.
        behaviour.request.send_request(friends, BlockRequest());
    }

    let topic = Topic::new("Block");

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
    /*
    swarm.listen_on(
        "/ip4/127.0.0.1/tcp/8001/ws/p2p-webrtc-star"
            .parse()
            .unwrap(),
    )
    .unwrap();*/

    // Kick it off.
    let behaviour = swarm.behaviour_mut();
    //This is the loop i was talking about for game events.
    loop {
        match behaviour.reciever.try_recv() {
            Ok(request) => match request {
                Request::AddFriend(peer) => {
                    behaviour.friends.add_friend(peer);
                    db::put(
                        "friends".to_string(),
                        db::serialize(&behaviour.friends).expect("serde error"),
                    );
                }
                Request::RemoveFriend(peer) => {
                    behaviour.friends.remove_friend(peer);
                    db::put(
                        "friends".to_string(),
                        db::serialize(&behaviour.friends).expect("serde error"),
                    );
                }
                Request::SendTransaction(pubk, ammount) => {
                    if let identity::Keypair::Secp256k1(k) = local_key.clone() {
                        if let Some(AccountInfo { value: _, nonce }) =
                            behaviour.accounts.value.account(&local_peer_id)
                        {
                            behaviour
                                .gossipsub
                                .publish(
                                    Topic::new("Transaction"),
                                    db::serialize(&Transaction::new(
                                        k,
                                        pubk,
                                        ammount,
                                        nonce.clone(),
                                    ))
                                    .expect("serde Error")
                                    .as_bytes(),
                                )
                                .expect("pub Error");
                        }
                    }
                }
                Request::CreateBlock() => {
                    let block = Block::default();
                    behaviour
                        .gossipsub
                        .publish(
                            Topic::new("Block"),
                            db::serialize(&block).expect("serde Error").as_bytes(),
                        )
                        .expect("pub Error");
                    crate::networks::events::update_accounts(
                        &mut behaviour.accounts,
                        block.clone(),
                    );
                    behaviour.last_block = block.clone();
                    behaviour.mempool.rm(&block.tx);
                }
                Request::NetworkEvent(e) => {
                    behaviour
                        .gossipsub
                        .publish(
                            Topic::new("move"),
                            db::serialize(&e).expect("serde Error").as_bytes(),
                        )
                        .expect("pub Error");
                }
            },
            Err(_) => (),
        }
        /*
        select! {
            line = stdin.select_next_some() => input::handle_input_line(behaviour, line.expect("Stdin not to close"), topic.clone()),
            event = behaviour.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {:?}", address);

                },
                _ => {}
            }
        }*/
        //Only retains blocks of certain condition. Right now always returns true. Not important
        behaviour.kademlia.store_mut().retain(validate::validate);
    }
}
