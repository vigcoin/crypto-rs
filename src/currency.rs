extern crate dirs;

pub use super::config::{Config};

pub struct Currency {
  pub config: Config,
  pub isHomeDir: bool,
}

impl Currency {
  // Get Files
  fn getFiles(&self, name: &str) -> String {
    let mut path;
    if cfg!(unix) && self.isHomeDir {
      path = dirs::home_dir().unwrap();
      path.push(format!(".{}", self.config.coinName));
    } else {
      path = dirs::data_dir().unwrap();
      path.push(self.config.coinName);
    }

    path.push(name);

    let filename = String::from(path.to_str().unwrap());
    return filename;
  }

  pub fn getBlockFile(&self) -> String {
    return self.getFiles(self.config.files.block);
  }

  pub fn getBlockCacheFile(&self) -> String {
    return self.getFiles(self.config.files.blockCache);
  }

  pub fn getBlockIndexFile(&self) -> String {
    return self.getFiles(self.config.files.blockIndex);
  }

  pub fn getBlockChainFile(&self) -> String {
    return self.getFiles(self.config.files.blockChain);
  }

  pub fn new(config: Config) -> Currency {
    Currency {
      config: config,
      isHomeDir: false,
    }
  }

  pub fn old(config: Config) -> Currency {
    Currency {
      config: config,
      isHomeDir: true,
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
 use super::super::config::{Config, CryptoNoteCoinFiles};
 use super::super::types::basic::Version;

  #[test]
  fn should_GetBlockFile() {
    let files = CryptoNoteCoinFiles::new([
      ("blocks.dat"),
      ("blockindexes.dat"),
      ("blockscache.dat"),
      ("blockchainindices.dat"),
    ]);
    let config = Config::new(
      files,
      "vigcoin",
      "aaa",
      Version {
        major: 1,
        minor: 0,
        patch: 0,
      },
    );

    let currency = Currency::old(config);
    let mut path = dirs::data_dir().unwrap();
    path.push(currency.config.coinName);
    assert!(currency.getFiles("blocks.dat") == currency.getBlockFile());
    assert!(currency.getFiles("blockindexes.dat") == currency.getBlockIndexFile());
    assert!(currency.getFiles("blockscache.dat") == currency.getBlockCacheFile());
    assert!(currency.getFiles("blockchainindices.dat") == currency.getBlockChainFile());
    println!("{}", currency.getBlockFile());
    println!("{}", currency.getBlockIndexFile());
    println!("{}", currency.getBlockCacheFile());
    println!("{}", currency.getBlockChainFile());
  }

  #[test]
  fn should_GetV2BlockFile() {
    let files = CryptoNoteCoinFiles::new([
      ("blocks.dat"),
      ("blockindexes.dat"),
      ("blockscache.dat"),
      ("blockchainindices.dat"),
    ]);
    let config = Config::new(
      files,
      "vigcoin",
      "aaa",
      Version {
        major: 2,
        minor: 0,
        patch: 0,
      },
    );

    let currency = Currency::new(config);
    let mut path = dirs::data_dir().unwrap();
    path.push(currency.config.coinName);
    assert!(currency.getFiles("blocks.dat") == currency.getBlockFile());
    assert!(currency.getFiles("blockindexes.dat") == currency.getBlockIndexFile());
    assert!(currency.getFiles("blockscache.dat") == currency.getBlockCacheFile());
    assert!(currency.getFiles("blockchainindices.dat") == currency.getBlockChainFile());
    println!("{}", currency.getBlockFile());
    println!("{}", currency.getBlockIndexFile());
    println!("{}", currency.getBlockCacheFile());
    println!("{}", currency.getBlockChainFile());
  }
}
