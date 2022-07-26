use super::transactions::{MemPool, Transaction};
use crate::networks::db::db;
use components::struc;
use components::struc::ValueList;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    /// Reference to the previous block in the chain.
    pub prev_blockhash: String,
    pub bounty: PeerId,
    /// The timestamp of the block, as claimed by the miner.
    pub time: SystemTime,
    // The nonce, selected to obtain a low enough blockhash.
    pub tx: Vec<Transaction>,
    pub world: String,
}

impl Block {
    pub fn default() -> Block {
        Block::new(
            last_block().hash(),
            Vec::<Transaction>::with_capacity(100),
            "".to_string(),
        )
    }
    pub fn new(prev: String, t: Vec<Transaction>, w: String) -> Block {
        Block {
            prev_blockhash: prev,
            time: SystemTime::now(),
            bounty: struc::BountyList::get_bounty(),
            tx: t,
            world: w,
        }
    }
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        println!("{:?}", result);
        result
    }
    pub fn generate_next_block(&self, mut mem: MemPool, mut value: &ValueList) -> (Block, MemPool) {
        let mut tx = Vec::<Transaction>::with_capacity(100);
        while tx.len() < 100 {
            if let Some(txt) = mem.pop() {
                if txt.verify_transaction_sig() && txt.verify_value(&mut value) {
                    tx.push(txt);
                }
            } else {
                break;
            }
        }
        (Block::new(last_block().hash(), tx, "".to_string()), mem)
    }
    pub fn validate(&self) {}
}
pub fn last_block() -> Block {
    let last: Block = serde_json::from_str(&db::get("last".to_string())).unwrap();
    last
}
