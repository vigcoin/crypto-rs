use super::basic::*;
use super::transaction::*;

pub struct BlockHeader {
  version: Version,
  nonce: u32,
  timestamp: u64,
  preHash: Hash,
}

pub struct Block {
  header: BlockHeader,
  transaction: Transaction,
  transactionHashes: Vec<Hash>,
}

pub struct BlockEntry {
  block:Block,
  height: u32,
  cumulativeSize: u64,
  cumulativeDifficulty: Difficulty,
  alreadyGeneratedCoins: u64,
  transactions: Vec<TransactionEntry>,
}