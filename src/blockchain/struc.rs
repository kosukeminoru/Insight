use crate::db::db;
use libp2p::PeerId;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
//use rand_seeder::{Seeder, SipHasher};
use crate::blockchain;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Serialize, Deserialize, Debug)]
pub struct BountyList {
    pub list: HashMap<PeerId, u32>,
}
impl BountyList {
    pub fn increase_bounty(&mut self, peer: PeerId) {
        let count = self.list.entry(peer).or_insert(0);
        let x = *count;
        self.list.insert(peer, x + 1);
    }
    pub fn decrease_bounty(&mut self, peer: PeerId) {
        let count = self.list.entry(peer).or_insert(0);
        let x = *count;
        self.list.insert(peer, x - 1);
    }
    pub fn get_bounty() -> PeerId {
        PeerId::random()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineList {
    pub list: HashMap<PeerId, bool>,
}
impl OnlineList {
    pub fn update(&mut self, peer: PeerId, online: bool) {
        self.list.insert(peer, online);
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AccountInfo {
    value: f32,
    nonce: u32,
}
impl AccountInfo {
    pub fn default() -> AccountInfo {
        AccountInfo {
            value: 0.0,
            nonce: 1,
        }
    }
    pub fn value_add(&mut self, v: f32) {
        self.value += v;
    }
    pub fn value_sub(&mut self, v: f32) {
        self.value -= v;
    }
    pub fn nonce_inc(&mut self) {
        self.nonce += 1;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueList {
    pub list: HashMap<PeerId, AccountInfo>,
}
impl ValueList {
    pub fn account(&mut self, peer: &PeerId) -> Option<&AccountInfo> {
        self.list.get(peer)
    }
    pub fn add(&mut self, peer: PeerId, v: f32) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.value_add(v);
        self.list.insert(peer, x);
    }
    pub fn sub(&mut self, peer: PeerId, v: f32) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.value_sub(v);
        self.list.insert(peer, x);
    }
    pub fn nonce_increment(&mut self, peer: PeerId) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.nonce_inc();
        self.list.insert(peer, x);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    pub bounty_list: BountyList,
    pub active: OnlineList,
    pub value: ValueList,
}
