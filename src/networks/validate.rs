// validate the record store and only retain records that do xyz

use libp2p::kad::{record::Key, Record};

pub fn validate(_a: &Key, _b: &mut Record) -> bool {
    true
}
