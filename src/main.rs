// Using game as a separate crate
use components::peers;
use components::struc::NetworkEvent;
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
    let (game_send, net_recieve) = unbounded::<Request>();
    let (net_send, game_recieve) = unbounded::<NetworkEvent>();
    let my_future = networks::protocol::into_protocol(private, peerid, s, net_recieve, net_send);
    thread::spawn(move || block_on(my_future).expect("Thread Spawn Error"));
    components::game::simulation::run(r, game_send, game_recieve);
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
