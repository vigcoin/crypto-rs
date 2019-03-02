extern crate hex;

pub use super::currency::Currency;

pub use super::types::block::Block;

pub struct BlockChain {
  currency: Currency,
}

impl BlockChain {
  pub fn new(currency: Currency) -> BlockChain {
    BlockChain { currency: currency }
  }

  pub fn genesis(&self) -> Block {
    let mut block: Block = Block::new();

    let bytes = hex::decode(self.currency.config.genesisCoinBaseTxHex).unwrap();
    // block.transaction.
    block.header.version = self.currency.config.version.clone();
    block.header.timestamp = 0;
    block.header.nonce = 70;
    block
  }
}


#[cfg(test)]

mod tests {
  use super::*;
 use super::super::config::{Config, CryptoNoteCoinFiles};
 use super::super::types::basic::Version;

  #[test]
  fn should_create_blockchain() {
        let files = CryptoNoteCoinFiles::new([
      ("blocks.dat"),
      ("blockindexes.dat"),
      ("blockscache.dat"),
      ("blockchainindices.dat"),
    ]);
    let config = Config::new(
      files,
      "vigcoin",
      "013c01ff000101029b2e4c0281c0b02e7c53291a94d1d0cbff8883f8024f5142ee494ffbbd0880712101398fb9ec4e76aeef124dfb779de715022efd619e63d7516f8b1470266f5da1fd",
      Version {
        major: 1,
        minor: 0,
        patch: 0,
      },
    );

    let currency = Currency::old(config);
    let blockchain = BlockChain::new(currency);
    let block = blockchain.genesis();
    assert!(block.header.timestamp == 0);
    assert!(block.header.nonce == 70);
    assert!(block.header.version.major == 1);
    assert!(block.header.version.minor == 0);
    assert!(block.header.version.patch == 0);
  }
}
