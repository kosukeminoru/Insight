use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::time::SystemTime;
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    /// Reference to the previous block in the chain.
    pub prev_blockhash: String,
    /// The timestamp of the block, as claimed by the miner.
    pub time: SystemTime,
    // The nonce, selected to obtain a low enough blockhash.
    pub signers: Vec<PeerId>,
    pub txdata: Vec<Transaction>,
    pub movedata: Vec<Move>,
}
impl BlockHeader {
    pub fn block_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        println!("{:?}", result);
        result
    }
}
/*pub struct Block {
    pub header: BlockHeader,
    pub txdata: Vec<Transaction>,
    pub movedata: Vec<Move>,
}*/
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
