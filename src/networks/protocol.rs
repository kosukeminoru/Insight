use super::db::db;
use crate::blockchain::block::block_hash;
use crate::blockchain::block::Block;
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
use components::struc::Accounts;
use components::struc::FriendsList;
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
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig},
    swarm::SwarmEvent,
    PeerId, Swarm,
};
use std::arch::x86_64::_SIDD_MOST_SIGNIFICANT;
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
) -> Result<(), Box<dyn Error>> {
    let last_block: Block = db::try_get_last_block();
    let friends: FriendsList = db::try_get_friends();
    let accounts: Accounts = db::try_get_accounts();
    println!("{:?}", local_peer_id);

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
            reciever,
            accounts,
            friends,
            last_block,
            boot_helper: BootHelper {
                temp_last: "".to_string(),
                old_last: "".to_string(),
                friends_last: Vec::new(),
            },
            node_type: NodeType::FullNode,
        };
        Swarm::new(transport, behaviour, local_peer_id)
    };
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip6/::0/tcp/0".parse()?)?;
    //swarm.listen_on("/ip4/192.168.1.197/tcp/54005".parse()?)?;
    swarm = zkademlia::boot(swarm);
    let behaviour = swarm.behaviour_mut();
    for friends in behaviour.friends.list() {
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
    loop {
        select! {
            line = stdin.select_next_some() => input::handle_input_line(&mut swarm.behaviour_mut(), line.expect("Stdin not to close"), topic.clone()),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {:?}", address);

                },
                _ => {}
            }
        }
        swarm
            .behaviour_mut()
            .kademlia
            .store_mut()
            .retain(validate::validate);
    }
}
