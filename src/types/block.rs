

use super::basic::*;
use super::transaction::Transaction;

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