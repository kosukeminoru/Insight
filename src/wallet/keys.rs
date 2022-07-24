use crate::db::db;
use libp2p::core::identity::secp256k1;
pub fn gen_key(gen: Option<&str>) -> Keypair {
    match gen {
        None => Keypair::generate(),
        Some(s) => Keypair::from(SecretKey::from_bytes(s.as_bytes())),
    }
}

pub fn put_key(pwd: str, keypair: Keypair) {
    db::put(pwd.to_string(), serde_json::to_string(&keypair));
}

pub fn get_key(pwd: str) -> Keypair {
    let key: Keypair = serde_json::from_str(&db::get(pwd.to_string())).unwrap();
}
