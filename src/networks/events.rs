use super::db::db;
use super::util;
use super::zrequest::BlockCodec;
use super::zrequest::BlockRequest;
use super::zrequest::BlockResponse;
use crate::blockchain::block::block_hash;
use crate::blockchain::block::Block;
use components::struc::Accounts;
use components::struc::FriendsList;
use components::struc::NetworkInfo;
use components::struc::Request;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use libp2p::core::PublicKey;
use libp2p::gossipsub;
use libp2p::gossipsub::GossipsubEvent;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::identify::Identify;
use libp2p::identify::IdentifyEvent;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::record::store::RecordStore;
use libp2p::kad::{record::Key, Quorum, Record};
use libp2p::kad::{
    AddProviderOk, InboundRequest, Kademlia, KademliaEvent, PeerRecord, PutRecordOk, QueryResult,
};
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseEvent;
use libp2p::request_response::RequestResponseMessage;
use libp2p::PeerId;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};

pub struct BootHelper {
    pub temp_last: String,
    pub old_last: String,
    pub friends_last: Vec<String>,
}
//custom out event
//two kademlias
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct MyBehaviour {
    pub gossipsub: libp2p::gossipsub::Gossipsub,
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: Identify,
    pub mdns: Mdns,
    pub request: RequestResponse<BlockCodec>,
    #[behaviour(ignore)]
    pub sender: Sender<NetworkInfo>,
    #[behaviour(ignore)]
    pub reciever: Receiver<Request>,
    #[behaviour(ignore)]
    pub accounts: Accounts,
    #[behaviour(ignore)]
    pub friends: FriendsList,
    #[behaviour(ignore)]
    pub last_block: Block,
    #[behaviour(ignore)]
    // change to temp_last
    pub boot_helper: BootHelper,
    #[behaviour(ignore)]
    pub node_type: NodeType,
}
pub enum NodeType {
    FullNode,
    LightNode,
    Client,
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<BlockRequest, BlockResponse>>
    for MyBehaviour
{
    fn inject_event(&mut self, event: RequestResponseEvent<BlockRequest, BlockResponse>) {
        match event {
            RequestResponseEvent::Message { peer: _, message } => match message {
                RequestResponseMessage::Request {
                    request_id: _,
                    request: _,
                    channel,
                } => {
                    self.request
                        .send_response(
                            channel,
                            BlockResponse(block_hash(&db::try_get_last_block())),
                        )
                        .expect("response error");
                }
                RequestResponseMessage::Response {
                    request_id: _,
                    response,
                } => {
                    let BlockResponse(hash) = response;
                    self.boot_helper.friends_last.push(hash);
                    if self.friends.list().len() == self.boot_helper.friends_last.len() {
                        self.boot_helper
                            .friends_last
                            .retain(|x| x != &"".to_string());
                        let freq = util::most_frequent(&self.boot_helper.friends_last, 1)[0]
                            .1
                            .to_string();
                        if freq != block_hash(&self.last_block) {
                            let mut most_frequent_hash = Vec::<String>::new();
                            most_frequent_hash.push(freq.clone());
                            self.boot_helper.temp_last = freq;
                            self.boot_helper.old_last = block_hash(&self.last_block);
                            self.boot_helper.friends_last = most_frequent_hash;
                            self.kademlia.get_record(
                                Key::new(&self.boot_helper.friends_last[0]),
                                Quorum::One,
                            );
                        } else {
                            let topic = Topic::new("Block");
                            self.gossipsub.subscribe(&topic).expect("Correct topic");
                        }
                    }
                }
            },
            RequestResponseEvent::OutboundFailure {
                peer: _,
                request_id: _,
                error: _,
            } => {
                self.boot_helper.friends_last.push("".to_string());
                if self.friends.list().len() == self.boot_helper.friends_last.len() {
                    self.boot_helper
                        .friends_last
                        .retain(|x| x != &"".to_string());
                    let freq = util::most_frequent(&self.boot_helper.friends_last, 1)[0]
                        .1
                        .to_string();
                    if freq != block_hash(&self.last_block) {
                        let mut most_frequent_hash = Vec::<String>::new();
                        most_frequent_hash.push(freq.clone());
                        self.boot_helper.temp_last = freq;
                        self.boot_helper.old_last = block_hash(&self.last_block);
                        self.boot_helper.friends_last = most_frequent_hash;
                        self.kademlia
                            .get_record(Key::new(&self.boot_helper.friends_last[0]), Quorum::One);
                    } else {
                        let topic = Topic::new("Block");
                        self.gossipsub.subscribe(&topic).expect("Correct topic");
                    }
                }
            }
            _ => (),
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
    // Called when `gossipsub` produces an event.
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let gossipsub::GossipsubEvent::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        } = event
        {
            //When recieved
            println!(
                "Got message: {} with id: {} from peer: {:?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            )
        }
    }
}
impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
                //self.gossipsub.add_explicit_peer(&peer_id);
            }
        }
    }
}
impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    // Called when `kademlia` produces an event.
    fn inject_event(&mut self, message: KademliaEvent) {
        match message {
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                QueryResult::GetProviders(Ok(ok)) => {
                    for peer in ok.providers {
                        println!(
                            "Peer {:?} provides key {:?}",
                            peer,
                            std::str::from_utf8(ok.key.as_ref()).unwrap()
                        );
                        println!("Peers: {:?}", ok.closest_peers);
                    }
                }
                QueryResult::GetProviders(Err(err)) => {
                    eprintln!("Failed to get providers: {:?}", err);
                }
                // more flexible get (Search Engine)
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord {
                        record: Record { key: _, value, .. },
                        ..
                    } in ok.records
                    {
                        if let Ok(recieved_block) = db::deserialize::<Block>(&value) {
                            //quick validation
                            if self.boot_helper.temp_last == block_hash(&recieved_block) {
                                if self.boot_helper.friends_last[0] == block_hash(&recieved_block) {
                                    // my last = friends_last block
                                    self.last_block = recieved_block.clone();
                                }
                                //adjust accounts
                                for txs in &recieved_block.tx {
                                    self.accounts.value.add(
                                        PeerId::from_public_key(
                                            &PublicKey::from_protobuf_encoding(&txs.data.recepient)
                                                .expect("PublicKey conversion failed"),
                                        ),
                                        txs.data.value,
                                    );
                                    self.accounts.value.sub(
                                        PeerId::from_public_key(
                                            &PublicKey::from_protobuf_encoding(&txs.data.sender)
                                                .expect("PublicKey conversion failed"),
                                        ),
                                        txs.data.value,
                                    );
                                    self.accounts.value.nonce_increment(PeerId::from_public_key(
                                        &PublicKey::from_protobuf_encoding(&txs.data.sender)
                                            .expect("PublicKey conversion failed"),
                                    ));
                                }
                                // should always succeed since deserialization occured prior
                                if let Ok(serial) = db::serialize(&recieved_block) {
                                    match self.node_type {
                                        NodeType::FullNode => {
                                            db::put(block_hash(&recieved_block), serial)
                                                .expect("DB error");
                                        }
                                        //can make Light Nodes uselfull here by storing different structure
                                        NodeType::LightNode => {
                                            db::put(
                                                block_hash(&recieved_block),
                                                block_hash(&recieved_block),
                                            )
                                            .expect("DB error");
                                        }
                                        _ => (),
                                    }
                                    if recieved_block.prev_blockhash != self.boot_helper.old_last {
                                        self.boot_helper.temp_last =
                                            recieved_block.prev_blockhash.clone();
                                        self.kademlia.get_record(
                                            Key::new(&recieved_block.prev_blockhash),
                                            Quorum::One,
                                        );
                                    } else {
                                        for friends in self.friends.list() {
                                            self.request.send_request(friends, BlockRequest());
                                        }
                                    }
                                    break;
                                }
                                break;
                            }
                        }
                    }
                }
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to get record: {:?}", err);
                }
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!(
                        "Successfully put record {:?}",
                        std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to put record: {:?}", err);
                }
                QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                    println!(
                        "Successfully put provider record {:?}",
                        std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::StartProviding(Err(err)) => {
                    eprintln!("Failed to put provider record: {:?}", err);
                }
                _ => {}
            },
            /*
            KademliaEvent::RoutingUpdated { peer, old_peer, .. } => {
                let mut iter = self.kademlia.kbuckets();
                let mut count = 0;
                for p in iter {
                    //self.gossipsub.add_explicit_peer(p.node.key.preimage());
                    //println!("{:?}", p.node.key.preimage());
                    print!("new bucket: ");
                    println!("{:?}", p.num_entries());
                    println!("{:?}", count);
                    count += 1;
                }
            }*/
            KademliaEvent::InboundRequest { request } => match request {
                InboundRequest::AddProvider { record } => {
                    self.kademlia
                        .store_mut()
                        .add_provider(record.unwrap())
                        .expect("err!");
                }
                InboundRequest::PutRecord {
                    source: _,
                    connection: _,
                    record,
                } => {
                    self.kademlia
                        .store_mut()
                        .put(record.unwrap())
                        .expect("err!");
                }
                _ => println!("{:?}", request),
            },
            _ => (),
        }
    }
}
impl NetworkBehaviourEventProcess<IdentifyEvent> for MyBehaviour {
    fn inject_event(&mut self, _message: IdentifyEvent) {
        /*
        match message {
            IdentifyEvent::Received { peer_id, info } => {
                self.kademlia.add_address(&peer_id, info.listen_addrs);
            }
        }*/
    }
}

fn suggest() {
    println!("someting wong")
}
