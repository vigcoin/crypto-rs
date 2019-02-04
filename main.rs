mod types;
mod account;
use std::mem;

fn main() {
    assert_eq!(32, std::mem::size_of::<types::hash_t>());
    println!("{}", std::mem::size_of::<types::hash_t>());
    println!("{}", std::mem::size_of::<account::Account>());
    println!("Hello, world!");
}
