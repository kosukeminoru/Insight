#![windows_subsystem = "windows"]

// Using game as a separate crate

pub mod blockchain;
pub mod db;
pub mod game;
pub mod networks;
pub mod validation;
use futures::executor::block_on;
use libp2p::{identity, PeerId};
use std::time::SystemTime;

fn main() {
    /*
        let r = db::db::get("list".to_string());
        let r1 = db::db::get("last".to_string());
        let mut priva: identity::Keypair;
        if r1 == "".to_string() {
            let value = blockchain::block::BlockHeader {
                prev_blockhash: "".to_string(),
                time: SystemTime::now(),
                signers: vec![],
                txdata: vec![],
                movedata: vec![],
            };
            let send = serde_json::to_string(&value).unwrap();
            db::db::put("list".to_string(), send);
        }

        if db::db::get("priva".to_string()) == "".to_string() {
            priva = identity::Keypair::generate_ed25519();
            db::db::put(
                "priva".to_string(),
                serde_json::to_string(&priva.to_protobuf_encoding().unwrap()).unwrap(),
            );
        } else {
            priva = identity::Keypair::from_protobuf_encoding(
                serde_json::from_str(&db::db::get("priva".to_string())).unwrap(),
            )
            .unwrap();
        }
        let peerid = PeerId::from(priva.public());
        if r == "".to_string() {
            db::db::put(
                "list".to_string(),
                serde_json::to_string(&blockchain::struc::Plist {
                    membersa: vec![blockchain::struc::Member {
                        peer: peerid.clone(),
                        active: true,
                        value: 0,
                    }],
                    membersn: vec![],
                })
                .unwrap(),
            )
        }
    */
    let priva = identity::Keypair::generate_ed25519();
    let peerid = PeerId::from(priva.public());
    let my_future = networks::nmain::ping(priva, peerid);
    block_on(my_future).expect("error");
}

//game::simulation::run();
//   let attempt: game::player::Attempt = serde_json::from_str(&db::db::get(String::from("tempattempt"))).unwrap();

/*
if validation::validation::run(attempt){

        println!("win");
    }
    else {
       println!("lose");
 }
*/
//}

/* println!("{:?}", db::db::get(String::from("tempattempt")));
let new: game::player::Attempt = serde_json::from_str(&db::db::get(String::from("tempattempt"))).unwrap();
    println!("{:?}",std::mem::size_of::<game::player::Attempt>());
    println!("{:?}", new);
    let new: game::player::MouseResource = serde_json::from_str(&db::db::get(String::from("1"))).unwrap();
    println!("{:?}", new);
    */
