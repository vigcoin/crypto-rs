use chrono::Utc;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
use rand::Rng;

pub struct Address {
    spend: PublicKey,
    view: PublicKey,
}

pub struct Keys {
    address: Address,
    spend: SecretKey,
    view: SecretKey,
}

pub struct Account {
    keys: Keys,
    prefix: u64,
    timestamp: u64,
}

pub fn unix_timestamp() -> u64 {
  return Utc::now().timestamp() as u64;
}

impl Address {
      fn new(spend: PublicKey,  view: PublicKey) -> Address {
      Address {
        spend: spend,
        view: view
      }
      }

}

impl Keys {
        fn new(address:Address, spend: SecretKey,  view: SecretKey) -> Keys {
      Keys {
        address: address,
        spend: spend,
        view: view,
      }
      }
}

impl Account {
    fn new(prefix: u64) -> Account {
        let mut spendRng: OsRng = OsRng::new().unwrap();
        let mut viewRng: OsRng = OsRng::new().unwrap();
        let spendKeypair: Keypair = Keypair::generate(&mut spendRng);
        let viewKeypair: Keypair = Keypair::generate(&mut viewRng);
        let address: Address = Address::new(spendKeypair.public, viewKeypair.public);
        let keys: Keys = Keys::new(address, spendKeypair.secret, viewKeypair.secret);
        Account {
            prefix: prefix,
            timestamp: unix_timestamp(),
            keys: keys,
        }
    }
}
