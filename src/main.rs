// Using game as a separate crate
use components::peers;
use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;
use std::thread;
pub mod blockchain;
use components;
pub mod networks;
use components::struc::NetworkInfo;
use components::struc::Request;
use futures::executor::block_on;
use libp2p::{identity, PeerId};

fn main() {
    env_logger::init();
    let private: identity::Keypair =
        identity::Keypair::from_protobuf_encoding(&peers::P1KEY).expect("Decoding Error");
    let peerid: PeerId = PeerId::from(private.public());
    let (s, r) = bounded::<NetworkInfo>(1);
    let (send, recieve) = unbounded::<Request>();
    let my_future = networks::protocol::into_protocol(private, peerid, s, recieve);
    thread::spawn(move || block_on(my_future).expect("Thread Spawn Error"));
    components::game::simulation::run(r, send);
}

/*
let priva: identity::Keypair = identity::Keypair::generate_ed25519();
    let peerid: PeerId = PeerId::from(priva.public());
     */
/*
let priva: identity::Keypair = identity::Keypair::from_protobuf_encoding(&peers::P2KEY).unwrap();
let peerid: PeerId = PeerId::from(priva.public());
let priva: identity::Keypair = identity::Keypair::from_protobuf_encoding(&peers::P3KEY).unwrap();
let peerid: PeerId = PeerId::from(priva.public());
*/
