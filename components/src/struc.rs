use crate::peers;
use bevy::ecs::reflect::ReflectComponent;
use bevy::{ecs::reflect, prelude::Component, reflect::Reflect};
use libp2p::{identity::secp256k1::PublicKey, PeerId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use rand_seeder::{Seeder, SipHasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub value: f32,
    pub nonce: u32,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueList {
    pub list: HashMap<PeerId, AccountInfo>,
}
impl ValueList {
    pub fn account(&self, peer: &PeerId) -> Option<&AccountInfo> {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Accounts {
    pub bounty_list: BountyList,
    pub active: OnlineList,
    pub value: ValueList,
}
impl Accounts {
    pub fn default() -> Accounts {
        Accounts {
            bounty_list: BountyList {
                list: HashMap::<PeerId, u32>::new(),
            },
            active: OnlineList {
                list: HashMap::<PeerId, bool>::new(),
            },
            value: ValueList {
                list: HashMap::<PeerId, AccountInfo>::new(),
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendsList {
    list: Vec<PeerId>,
}
impl FriendsList {
    pub fn default() -> FriendsList {
        FriendsList {
            list: vec![
                PeerId::from_bytes(&peers::P1ID).unwrap(),
                PeerId::from_bytes(&peers::P2ID).unwrap(),
                PeerId::from_bytes(&peers::P3ID).unwrap(),
            ],
        }
    }
    pub fn list(&self) -> &Vec<PeerId> {
        &self.list
    }
    pub fn add_friend(&mut self, peer: PeerId) {
        self.list.push(peer);
    }
    pub fn remove_friend(&mut self, peer: PeerId) -> Option<PeerId> {
        let peer_pos = self.list.iter().position(|&x| x == peer);
        match peer_pos {
            Some(pos) => {
                self.list.remove(pos);
                return Some(peer);
            }
            None => None,
        }
    }
}

pub enum Request {
    AddFriend(PeerId),
    RemoveFriend(PeerId),
    NetworkEvent(NetworkEvent),
    SendTransaction(PublicKey, f32),
    CreateBlock(Vec<HashMap>, bevy::World),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInfo {
    accounts: Accounts,
    friends: FriendsList,
}
impl NetworkInfo {
    pub fn new(a: Accounts, f: FriendsList) -> NetworkInfo {
        NetworkInfo {
            accounts: a,
            friends: f,
        }
    }
    pub fn default() -> NetworkInfo {
        NetworkInfo {
            accounts: Accounts::default(),
            friends: FriendsList::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkEvent {
    pub player: PeerId,
    pub input: PlayerInput,
}
#[derive(Serialize, Deserialize, Debug, Component)]
pub struct LocalPlayer;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInput {
    pub key: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
pub struct PlayerID {
    pub id: PeerId,
}
impl Default for PlayerID {
    fn default() -> Self {
        PlayerID {
            id: PeerId::random(),
        }
    }
}
