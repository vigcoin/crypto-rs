use chrono::Utc;

use super::basic::{Difficulty, Hash, Version};
use super::transaction::*;

pub struct BlockHeader {
  pub version: Version,
  pub nonce: u32,
  pub timestamp: u64,
  pub preHash: Hash,
}

pub struct Block {
  pub header: BlockHeader,
  pub transaction: Transaction,
  pub transactionHashes: Vec<Hash>,
}

pub struct BlockEntry {
  block: Block,
  height: u32,
  cumulativeSize: u64,
  cumulativeDifficulty: Difficulty,
  alreadyGeneratedCoins: u64,
  transactions: Vec<TransactionEntry>,
}

impl BlockHeader {
  pub fn new() -> BlockHeader {
    BlockHeader {
      version: Version::default(),
      nonce: 0,
      timestamp: Utc::now().timestamp() as u64,
      preHash: Hash::default(),
    }
  }
}

impl Block {
  pub fn new() -> Block {
    let header = BlockHeader::new();
    Block {
      header: BlockHeader::new(),
      transaction: Transaction::new(),
      transactionHashes: vec![],
    }
  }
}
