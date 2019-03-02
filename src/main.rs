#![allow(non_snake_case)]

mod account;
mod types;
mod config;
mod currency;
mod blockchain;
// use std::mem;

extern crate chrono;
extern crate ed25519_dalek;
extern crate leb128;
extern crate rand;
extern crate rust_base58;
extern crate tiny_keccak;
extern crate dirs;

use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
use rand::Rng;

fn main() {
  let mut csprng: OsRng = OsRng::new().unwrap();
  // let keypair: Keypair = Keypair::generate(&mut csprng);

  // assert_eq!(32, std::mem::size_of::<types::hash_t>());
  // println!("{}", std::mem::size_of::<types::hash_t>());
  println!("{}", std::mem::size_of::<account::Account>());
  println!("Hello, world!");
  println!("{}", account::unix_timestamp());
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
