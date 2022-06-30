// validate the record store and only retain records that do xyz

use libp2p::kad::{record::Key, Record};

pub fn validate(a: &Key, b: &mut Record) -> bool {
    false
}
