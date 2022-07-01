use crate::networks::events::MyBehaviour;
use crate::networks::input;
use crate::networks::validate;
use async_std::{io, task};
use futures::{prelude::*, select};
use libp2p::gossipsub;
use libp2p::gossipsub::{
    GossipsubMessage, IdentTopic as Topic, MessageAuthenticity, MessageId, ValidationMode,
};
use libp2p::kad;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::BootstrapOk;
use libp2p::kad::Kademlia;
use libp2p::kad::QueryInfo;
use libp2p::multiaddr::Multiaddr;
use libp2p::{
    development_transport, identity,
    mdns::{Mdns, MdnsConfig},
    swarm::SwarmEvent,
    PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::Duration;

pub async fn start_protocol(
    local_key: identity::Keypair,
    local_peer_id: PeerId,
) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random key for ourselves.

    println!("{:?}", local_peer_id);
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol.
    let transport = development_transport(local_key.clone()).await?;
    let topic = Topic::new("test-net");
    // We create a custom network behaviour that combines Kademlia and mDNS.

    // Create a swarm to manage peers and events.
    let mut swarm = {
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };
        // Set a custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
            .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the
            // same content will be propagated.
            .build()
            .expect("Valid config");
        // build a gossipsub network behaviour
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .expect("Correct configuration");

        // subscribes to our topic
        gossipsub.subscribe(&topic).unwrap();

        // Create a Kademlia behaviour.
        let mut config: kad::KademliaConfig = kad::KademliaConfig::default();
        config.set_replication_interval(Some(Duration::from_secs(5 * 60)));
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::with_config(local_peer_id, store, config);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let behaviour = MyBehaviour {
            gossipsub,
            kademlia,
            mdns,
        };

        Swarm::new(transport, behaviour, local_peer_id)
    };

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns.
    //swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    //
    swarm.listen_on("/ip4/10.150.108.167/tcp/51736".parse()?)?;
    let address: Multiaddr =
        "/ip4/10.150.108.167/tcp/51736/p2p/12D3KooWCP6NcrcxmcAuCoHF5nya7f8GjojHy1DP8nKqQvbzbhvm"
            .parse()
            .unwrap();
    swarm
        .behaviour_mut()
        .kademlia
        .add_address(&PeerId::try_from_multiaddr(&address).unwrap(), address);
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
                if let QueryInfo::Bootstrap { peer, remaining } = qi {
                    println!("{:?}", peer);
                    println!("{:?}", remaining);
                } else {
                    println!("wat");
                    break;
                }
            }
            Err(_) => break,
        }
    }
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

//Split Gossip and kademlia
