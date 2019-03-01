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
