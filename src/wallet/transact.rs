use super::keys;
use crate::blockchain::transactions;

pub fn create_tx(keypair: Keypair, t: PublicKey, v: f32, n: u32) -> Transaction {
    Transaction::new(keypair, t, v, n);
}
pub fn create_tx_from_password(p: str, t: PublicKey, v: f32, n: u32) {
    Transaction::new(keys::get_key(p), t, v, n);
}
