#[allow(non_snake_case_functions)]
extern crate dirs;

pub use super::config::{Config, CryptoNoteCoinFiles};

use std::path::PathBuf;

pub struct Currency {
  config: Config,
}

impl Currency {
  fn getBlockFile(&self) -> String {
    let mut path = dirs::data_dir().unwrap();
    path.push(format!(".{}", self.config.coinName));
    path.push(self.config.files.block);

    let filename = String::from(path.to_str().unwrap());
    return filename;
  }
  fn new(config: Config) -> Currency {
    Currency { config: config }
  }
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn should_GetBlockFile() {
    let files = CryptoNoteCoinFiles::new([("a"), ("b"), ("c"), ("d"), ("e"), ("f")]);
    let config = Config::new(files, "vigcoin");

    let currency = Currency::new(config);
    let mut path = dirs::data_dir().unwrap();
    path.push(format!(".{}", currency.config.coinName));
    path.push(currency.config.files.block);
    let filename = String::from(path.to_str().unwrap());
    assert!(filename == currency.getBlockFile());
    println!("{}", filename);
  }
}
