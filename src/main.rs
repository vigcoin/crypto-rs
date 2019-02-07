mod account;
use std::mem;

extern crate rand;
extern crate ed25519_dalek;
extern crate chrono;

use rand::Rng;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

fn main() {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate(&mut csprng);

    // assert_eq!(32, std::mem::size_of::<types::hash_t>());
    // println!("{}", std::mem::size_of::<types::hash_t>());
    println!("{}", std::mem::size_of::<account::Account>());
    println!("Hello, world!");
    println!("{}", account::unix_timestamp());
}
