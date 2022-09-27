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
pub mod game_world;
use game_world::ggrs_rollback::simulation;

/*
The code is separated into two parts. Components and Src. Both use the SAME local files downloaded by cargo.toml (share a target space)
Components contains a file called stuc and peers. Struc contains the useful structs and peers are 3 hard coded peers with private keys.
 */
fn main() {
    //env_logger::init();
    //Using peer 1
    /*
    let priva: identity::Keypair =
        identity::Keypair::from_protobuf_encoding(&peers::P1KEY).expect("Decoding Error");
    let peerid: PeerId = PeerId::from(priva.public());*/

    let priva: identity::Keypair = identity::Keypair::generate_ed25519();
    let peerid: PeerId = PeerId::from(priva.public());
    //simulation::run();
    //NetworkInfo (Friends list and accounts)

    let (s, r) = bounded::<NetworkInfo>(1);
    // //GameEvents
    let (game_send, net_recieve) = unbounded::<Request>();
    // //supposed to be for player movement
    let (net_send, game_recieve) = unbounded::<NetworkEvent>();
    let my_future = networks::protocol::into_protocol(priva, peerid, s, net_recieve, net_send);
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
