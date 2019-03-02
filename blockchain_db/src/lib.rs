use std::error::Error as StdError;

// pub mod error;
// pub mod lmdb;

pub trait BlockchainDB {
    // Constructor
    // fn new(path: &std::path::Path) -> Result<Box<Self>, Error>;

    // DB Operations
    fn open();
    fn is_open() -> bool;
    fn is_read_only();
    fn close();
    fn sync();
    fn set_safe_sync_mode(state: bool);
    fn reset();
    fn size();
    fn fixup();

    // Batch Operations
    // fn start_batch() -> Result<(), Error>;
    // fn stop_batch() -> Result<(), Error>;

    // Block
    fn add_block();
    fn get_block_by_height();
    fn get_block_by_hash();
    fn get_cumulative_difficulty();
    fn get_height();
    fn pop_block();

    // Confirmed Transactions
    fn add_transaction();
    fn get_transaction();

    // Unconfirmed Transactions
    fn add_txpool_transaction();
    fn get_txpool_transaction();
    fn get_txpool_transaction_count();
    fn remove_txpool_transaction();

    // Key Image
    fn has_key_image();
}
