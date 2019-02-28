pub struct CryptoNoteCoinFiles {
  pub block: &'static str,
  blockIndex: &'static str,
  blockCache: &'static str,
  pool: &'static str,
  p2p: &'static str,
  miner: &'static str,
}

pub struct Config {
  pub coinName: &'static str,
  pub files: CryptoNoteCoinFiles,
}

impl Config {
  pub fn new(files: CryptoNoteCoinFiles, coinName: &'static str) -> Config {
    Config {
      coinName: coinName,
      files: files,
    }
  }
}

impl CryptoNoteCoinFiles {
  pub fn new(files: [&'static str; 6]) -> CryptoNoteCoinFiles {
    CryptoNoteCoinFiles {
      block: files[0],
      blockIndex: files[1],
      blockCache: files[2],
      pool: files[3],
      p2p: files[4],
      miner: files[5],
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  #[test]
  fn should_create_coin_files() {
    let files = CryptoNoteCoinFiles::new([("a"), ("b"), ("c"), ("d"), ("e"), ("f")]);
    assert!(files.block == "a");
    assert!(files.blockIndex == "b");
    assert!(files.blockCache == "c");
    assert!(files.pool == "d");
    assert!(files.p2p == "e");
    assert!(files.miner == "f");
    let config = Config::new(files, "vigcoin");
    assert!(config.coinName == "vigcoin");
    // assert!(config.files == files);
  }
}
