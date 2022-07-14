#![windows_subsystem = "windows"]

// Using game as a separate crate
use std::sync::mpsc;

use std::thread;

pub mod blockchain;
pub mod db;
pub mod game;
pub mod networks;

use futures::executor::block_on;
use libp2p::{identity, PeerId};

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
    /*
    let priva;
    if db::db::get("priva".to_string()) == "".to_string() {
        priva = identity::Keypair::generate_ed25519();
        db::db::put(
            "priva".to_string(),
            serde_json::to_string(&priva.to_protobuf_encoding().unwrap()).unwrap(),
        );
    } else {
        let y = &db::db::get("priva".to_string());
        // let x = serde_json::from_str(y).unwrap();
        let x: [u8; 68] = [
            8, 1, 18, 64, 236, 219, 78, 215, 40, 219, 195, 32, 155, 130, 105, 2, 31, 197, 107, 68,
            180, 113, 242, 11, 55, 254, 89, 219, 224, 73, 147, 124, 229, 211, 138, 11, 38, 25, 174,
            72, 28, 220, 126, 249, 123, 12, 164, 200, 89, 111, 56, 135, 128, 88, 250, 164, 86, 74,
            172, 121, 106, 120, 35, 196, 229, 115, 199, 174,
        ];
        println!("{:?}", y);
        priva = identity::Keypair::from_protobuf_encoding(&x).unwrap();
    }
    */
    //random key

    // for boot nodes. Create by above^^
    let _x: [u8; 68] = [
        8, 1, 18, 64, 236, 219, 78, 215, 40, 219, 195, 32, 155, 130, 105, 2, 31, 197, 107, 68, 180,
        113, 242, 11, 55, 254, 89, 219, 224, 73, 147, 124, 229, 211, 138, 11, 38, 25, 174, 72, 28,
        220, 126, 249, 123, 12, 164, 200, 89, 111, 56, 135, 128, 88, 250, 164, 86, 74, 172, 121,
        106, 120, 35, 196, 229, 115, 199, 174,
    ];
    //let priva = identity::Keypair::from_protobuf_encoding(&x).unwrap();

    // let my_future = networks::protocol::start_protocol(priva, peerid);
    // block_on(my_future).expect("error");
    let priva: identity::Keypair = identity::Keypair::generate_ed25519();
    let peerid: PeerId = PeerId::from(priva.public());
    let (tx, _rx) = mpsc::channel::<String>();
    let my_future = networks::protocol::start_protocol(priva, peerid, tx);
    thread::spawn(move || block_on(my_future).expect("heyo"));
    /*
    loop {
        match rx.try_recv() {
            Ok(key) => println!("Received: {}", key),
            Err(TryRecvError::Empty) => println!("Channel empty"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        sleep(1000);
    }*/
    /*
    let attempt: game::player::Attempt =
        serde_json::from_str(&db::db::get(String::from("tempattempt"))).unwrap();*/
    //validation::validation::run(attempt);*/
    //thread::spawn(move || validation::validation::run(attempt));
    game::simulation::run();
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
