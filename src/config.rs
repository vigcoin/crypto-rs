pub struct CryptoNoteCoinFiles {
  pub block: &'static str,
  pub blockIndex: &'static str,
  pub blockCache: &'static str,
  pub blockChain: &'static str,
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
  pub fn new(files: [&'static str; 4]) -> CryptoNoteCoinFiles {
    CryptoNoteCoinFiles {
      block: files[0],
      blockIndex: files[1],
      blockCache: files[2],
      blockChain: files[3],
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  #[test]
  fn should_create_coin_files() {
    let files = CryptoNoteCoinFiles::new([("a"), ("b"), ("c"), ("d")]);
    assert!(files.block == "a");
    assert!(files.blockIndex == "b");
    assert!(files.blockCache == "c");
    assert!(files.blockChain == "d");
    let config = Config::new(files, "vigcoin");
    assert!(config.coinName == "vigcoin");
    // assert!(config.files == files);
  }
}
