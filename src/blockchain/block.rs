use crate::blockchain::struc;
use crate::db::db;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::time::SystemTime;
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    /// Reference to the previous block in the chain.
    pub prev_blockhash: String,
    pub bounty: PeerId,
    /// The timestamp of the block, as claimed by the miner.
    pub time: SystemTime,
    // The nonce, selected to obtain a low enough blockhash.
    pub tx_hash: String,
    pub world: String,
}

impl BlockHeader {
    pub fn default() -> BlockHeader {
        BlockHeader::new(last_block().hash(), "".to_string(), "".to_string())
    }
    pub fn new(prev: String, tx: String, w: String) -> BlockHeader {
        BlockHeader {
            prev_blockhash: prev,
            time: SystemTime::now(),
            bounty: struc::BountyList::get_bounty(),
            tx_hash: tx,
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
}
pub fn last_block() -> BlockHeader {
    let last: BlockHeader = serde_json::from_str(&db::get("last".to_string())).unwrap();
    last
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: Option<PeerId>,
    to: Vec<PeerId>,
    value: Vec<f32>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    who: PeerId,
    to: u32,
}
