// Using game as a separate crate
use crossbeam_channel::bounded;
use game_components::peers;
use std::thread;
pub mod blockchain;
use game_components;
pub mod networks;
use futures::executor::block_on;
use game_components::struc::NetworkInfo;
use libp2p::{identity, PeerId};
fn main() {
    let priva: identity::Keypair =
        identity::Keypair::from_protobuf_encoding(&peers::P1KEY).unwrap();
    let peerid: PeerId = PeerId::from(priva.public());
    let (s, r) = bounded::<NetworkInfo>(1);
    let my_future = networks::protocol::start_protocol(priva, peerid, s);
    thread::spawn(move || block_on(my_future).expect("heyo"));
    game_components::game::simulation::run(r);
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
