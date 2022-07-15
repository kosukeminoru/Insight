/*
use crate::db::db;
use libp2p::PeerId;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
//use rand_seeder::{Seeder, SipHasher};
use crate::blockchain;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plist {
    pub membersa: Vec<Member>,
    pub membersn: Vec<Member>,
}
impl Plist {
    pub fn get_bounty(&self) -> PeerId {
        let last_block: blockchain::block::BlockHeader =
            serde_json::from_str(&db::get(String::from("last"))).unwrap();
        let mut rng: Pcg64 = Seeder::from(last_block.block_hash()).make_rng();
        self.membersa[rng.gen_range(0..self.membersa.len())].peer
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub peer: PeerId,
    pub active: bool,
    pub value: u128,
}
*/
