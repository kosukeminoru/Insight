use crate::networks::events::MyBehaviour;
use crate::networks::input;
use crate::networks::transport;
use crate::networks::validate;
use crate::networks::zgossipsub;
use crate::networks::zkademlia;
use async_std::{io, task};
use futures::{prelude::*, select};
use libp2p::gossipsub;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::identify::{Identify, IdentifyConfig};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::Kademlia;
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig},
    swarm::SwarmEvent,
    PeerId, Swarm,
};
use std::error::Error;
use std::sync::mpsc::Sender;
pub async fn start_protocol(
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    sender: Sender<String>,
) -> Result<(), Box<dyn Error>> {
    env_logger::init();

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
        let behaviour = MyBehaviour {
            gossipsub,
            kademlia,
            identify,
            mdns,
        };
        Swarm::new(transport, behaviour, local_peer_id)
    };

    let topic = Topic::new("Block");
    swarm
        .behaviour_mut()
        .gossipsub
        .subscribe(&topic)
        .expect("Correct topic");

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
    /*
    swarm.listen_on(
        "/ip4/127.0.0.1/tcp/8001/ws/p2p-webrtc-star"
            .parse()
            .unwrap(),
    )
    .unwrap();*/
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip6/::0/tcp/0".parse()?)?;
    //swarm.listen_on("/ip4/192.168.1.197/tcp/54005".parse()?)?;
    swarm = zkademlia::boot(swarm);
    // Kick it off.
    loop {
        select! {
            line = stdin.select_next_some() => input::handle_input_line(&mut swarm.behaviour_mut(), line.expect("Stdin not to close"), topic.clone()),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {:?}", address);
                    sender.send("listenevent!".to_string()).unwrap();
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
