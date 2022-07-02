use super::events::MyBehaviour;
use libp2p::kad;
use libp2p::kad::record::store::{MemoryStore, MemoryStoreConfig};
use libp2p::kad::Kademlia;
use libp2p::kad::KademliaStoreInserts;
use libp2p::kad::QueryInfo;
use libp2p::multiaddr::Multiaddr;
use libp2p::Swarm;
use libp2p::{identity, PeerId};
use std::time::Duration;

pub fn create_kademlia(local_key: identity::Keypair) -> Kademlia<MemoryStore> {
    //name of dht (must be prefixed with /)
    const NAME: &[u8] = b"/insight";
    let local_peer_id = PeerId::from(local_key.public());
    let mut config: kad::KademliaConfig = kad::KademliaConfig::default();
    config.set_protocol_name(std::borrow::Cow::Borrowed(NAME));
    config.set_query_timeout(Duration::from_secs(3 * 60));
    config.set_record_ttl(None);
    config.set_record_filtering(KademliaStoreInserts::FilterBoth);
    config.set_publication_interval(None);
    config.set_replication_interval(Some(Duration::from_secs(5 * 60)));
    config.set_provider_record_ttl(None);
    config.set_provider_publication_interval(None);
    config.set_max_packet_size(usize::MAX);
    let mem_config = MemoryStoreConfig {
        max_records: usize::MAX,
        max_value_bytes: usize::MAX,
        max_provided_keys: 20,
        max_providers_per_key: 20,
    };
    let store = MemoryStore::with_config(local_peer_id, mem_config);
    let kademlia = Kademlia::with_config(local_peer_id, store, config);
    kademlia
}

pub fn boot(mut swarm: Swarm<MyBehaviour>) -> Swarm<MyBehaviour> {
    //boot nodes
    let address: Multiaddr =
        "/ip4/10.150.108.167/tcp/51736/p2p/12D3KooWCP6NcrcxmcAuCoHF5nya7f8GjojHy1DP8nKqQvbzbhvm"
            .parse()
            .unwrap();
    swarm
        .behaviour_mut()
        .kademlia
        .add_address(&PeerId::try_from_multiaddr(&address).unwrap(), address);
    //begin bootstrap
    let r = swarm.behaviour_mut().kademlia.bootstrap();
    loop {
        match r {
            Ok(bootstrap) => {
                let t = swarm
                    .behaviour_mut()
                    .kademlia
                    .query_mut(&bootstrap)
                    .unwrap();
                let qi = t.info();
                //continue until error or there are none to bootstrap left
                if let QueryInfo::Bootstrap { peer: _, remaining } = qi {
                    match remaining {
                        None => break,
                        _ => (),
                    }
                } else {
                    println!("wat");
                    break;
                }
            }
            Err(_) => break,
        }
    }
    swarm
}
