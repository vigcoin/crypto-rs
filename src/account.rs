use chrono::Utc;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
use rand::Rng;

use leb128;
use keccak;
use hex;
use rust_base58::{ToBase58, FromBase58};

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
    // address: string,
    keys: Keys,
    prefix: u64,
    timestamp: u64,
}

pub fn unix_timestamp() -> u64 {
    return Utc::now().timestamp() as u64;
}

impl Address {
    fn new(spend: PublicKey, view: PublicKey) -> Address {
        Address {
            spend: spend,
            view: view,
        }
    }
}

impl Keys {
    fn new(address: Address, spend: SecretKey, view: SecretKey) -> Keys {
        Keys {
            address: address,
            spend: spend,
            view: view,
        }
    }
}

impl Account {
    fn getAddress(&self)-> String {
        let mut tag = vec![];
        leb128::write::unsigned(&mut tag, self.prefix);
    // let tag = hex::encode(v);
    //     println!("{:?}", tag);
let spendArray: Vec<u8> = self.keys.address.spend.to_bytes().to_vec();
let viewArray: Vec<u8> = self.keys.address.view.to_bytes().to_vec();
let temp = [];
let given = [&temp, tag.as_slice(), spendArray.as_slice(), viewArray.as_slice()].concat();
let mut data = [0u64; 25];
let decoded = given.as_slice();
        println!("{}", decoded.len() );

for i in 0..25 {
    data[i] = 0;
    for j in 0..8 {
        println!("{}", i*8 + j );
        if (decoded.len() <= i * 8 + j) {
            break;
        }
        data[i] = decoded[i*8 + j].into();
        data[i] <<= 8;
    }
}

keccak::f1600(&mut data);
let mut checksum: [u8;4] = [0, 0, 0, 0];

let mut firstData: u32 = (data[0] >> 8 * 4) as u32;
let mut k = 3;
while k >= 0 {
    checksum[k] = (firstData & 0xFF) as u8;
    firstData >>= 8;
    if (k > 0) {
    k = k - 1;
    } else {
        break;
    }

}


let temp2 = self.prefix.to_le_bytes();

let preBase58 = [&temp2, decoded, checksum.to_vec().as_slice()].concat();

let beforeB58 = preBase58.as_slice();


    // println!("{:?}", given);
    println!("{:x?}", checksum);
    // println!("{:?}", tag);
    return preBase58.to_base58();
    //   std::string buf = varint::get(tag);
    //   buf += data;
    //   crypto::hash_t hash = crypto::cn_fast_hash(buf.data(), buf.size());
    //   const char* hash_data = reinterpret_cast<const char*>(&hash);
    //   buf.append(hash_data, addr_checksum_size);
    //   return encode(buf);
    }
    fn new(prefix: u64) -> Account {
        let mut spendRng: OsRng = OsRng::new().unwrap();
        let mut viewRng: OsRng = OsRng::new().unwrap();
        // assert!(spendRng != viewRng);
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

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn should_get_current_time() {
        let now1: u64 = unix_timestamp();
        assert!(now1 > 10000);
    }

    #[test]
    fn should_create_account() {
        let prefix = 0xBB;
        let acc: Account = Account::new(prefix);
        let now1: u64 = unix_timestamp();

        assert!(acc.prefix == prefix);
        assert!(acc.timestamp - now1 < 10);
        println!("{:x?}", acc.keys.spend);
        println!("{:?}", acc.getAddress());
    }
}
