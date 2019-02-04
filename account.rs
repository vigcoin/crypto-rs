use std::time::{Duration, SystemTime};
use types::account_keys_t;

pub struct Account {
    keys: account_keys_t,
    prefix: u64,
    timestamp: u64,
}

// impl Account {
//     fn generate() {

//     }
//     fn new(prefix: u64) -> Account {
//             let start = SystemTime::now();
//     let since_the_epoch = start.duration_since(UNIX_EPOCH);
//         Account {
//             prefix: prefix,
//             timestamp: since_the_epoch,
//             keys: 
//         }
//     }
// }
