//use rocksdb::{DB, Options};
// NB: db is automatically closed at end of lifetime
use crate::blockchain::block::Block;
use crate::components::struc::FriendsList;
use components::struc::Accounts;
use log::error;
use rocksdb::DBWithThreadMode;
use rocksdb::Error;
use rocksdb::MultiThreaded;
use rocksdb::Options;
use serde::Serialize;
use serde_json::Error as SError;
//Input value into the database with given key
//Key and Value should be serialized
pub fn put(key: String, value: String) -> Result<(), Error> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.increase_parallelism(3);
    let path = "database";
    pub type DB = DBWithThreadMode<MultiThreaded>;
    let db = DB::open(&opts, path).expect("DB Open failed");
    db.put(key, value)
}

//delete value from the database with given key
pub fn delete(key: String) {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.increase_parallelism(3);
    pub type DB = DBWithThreadMode<MultiThreaded>;
    let path = "database";
    let db = DB::open(&opts, path).expect("DB Open failed");
    db.delete(key).expect("delete failed");
}

//get value from the database with given key
pub fn get(key: String) -> Result<Option<Vec<u8>>, Error> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.increase_parallelism(3);
    pub type DB = DBWithThreadMode<MultiThreaded>;
    let path = "database";
    let db = DB::open(&opts, path).expect("DB Open failed");
    db.get(key)
}

pub fn serialize<T: Serialize>(value: &T) -> Result<String, SError> {
    serde_json::to_string(value)
}

pub fn deserialize<'a, T>(value: &'a Vec<u8>) -> Result<T, SError>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_str(std::str::from_utf8(value).expect("utf8 error"))
}
pub fn try_get(key: String, value: String) -> Option<Vec<u8>> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.increase_parallelism(3);
    pub type DB = DBWithThreadMode<MultiThreaded>;
    let path = "database";
    let db = DB::open(&opts, path).expect("DB Open failed");
    if let Ok(Some(value)) = db.get(key.clone()) {
        Some(value)
    } else {
        db.put(key, value).expect("put failed");
        None
    }
}

pub fn try_get_last_block() -> Block {
    if let Some(value) = try_get(
        "last".to_string(),
        serialize(&Block::default()).expect("serialize error"),
    ) {
        deserialize(&value).expect("deserialize block failed")
    } else {
        Block::default()
    }
}

pub fn try_get_friends() -> FriendsList {
    if let Some(value) = try_get(
        "friends".to_string(),
        serialize(&FriendsList::default()).expect("serialize error"),
    ) {
        deserialize(&value).expect("deserialize block failed")
    } else {
        FriendsList::default()
    }
}

pub fn try_get_accounts() -> Accounts {
    if let Some(value) = try_get(
        "accounts".to_string(),
        serialize(&Accounts::default()).expect("serialize error"),
    ) {
        deserialize(&value).expect("deserialize block failed")
    } else {
        Accounts::default()
    }
}
