use game_components::struc::ValueList;
use libp2p::core::identity::secp256k1::Keypair;
use libp2p::core::identity::secp256k1::PublicKey;
use libp2p::core::identity::PublicKey as PubKey;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    data: TxData,
    signature: Vec<u8>,
}
impl Transaction {
    pub fn new(k: Keypair, t: PublicKey, v: f32, n: u32) -> Transaction {
        let bytes = k.public().encode();
        let d = TxData::new(bytes, t.encode(), v, n);
        let hash_d = d.hash();
        Transaction {
            data: d,
            signature: k.secret().sign_hash(&*hash_d.as_bytes()).unwrap(),
        }
    }
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        result
    }
    pub fn verify_transaction_sig(&self) -> bool {
        let pubkey: PublicKey = PublicKey::decode(&self.data.sender).unwrap();
        let v_data = self.data.hash();
        let msg: &[u8] = &*v_data.as_bytes();
        let sig: &[u8] = &self.signature;
        pubkey.verify_hash(msg, sig)
    }
    pub fn verify_value(&self, values: &ValueList) -> bool {
        let pubkey: PublicKey = PublicKey::decode(&self.data.sender).unwrap();
        let acc = values
            .account(&PeerId::from_public_key(&PubKey::Secp256k1(pubkey)))
            .unwrap();
        if acc.value <= self.data.value && acc.nonce == self.data.nonce {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxData {
    #[serde(with = "BigArray")]
    sender: [u8; 33],
    #[serde(with = "BigArray")]
    recepient: [u8; 33],
    value: f32,
    nonce: u32,
}
impl TxData {
    pub fn new(from: [u8; 33], to: [u8; 33], val: f32, once: u32) -> TxData {
        TxData {
            sender: from,
            recepient: to,
            value: val,
            nonce: once,
        }
    }
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        result
    }
}

pub struct MemPool {
    txs: VecDeque<Transaction>,
}
impl MemPool {
    pub fn push(&mut self, tx: Transaction) {
        self.txs.push_back(tx);
    }
    pub fn pop(&mut self) -> Option<Transaction> {
        self.txs.pop_front()
    }
}
