use super::config::Config;

extern crate app_dirs;

use app_dirs::*;

pub struct Currency {
  config: Config,
}

impl Currency {
  fn new(config: Config) -> Currency {
    // const app: AppInfo = AppInfo {
    //   name: || { return config.name },
    //   author: "",
    // };
    Currency { config: config }
  }
}
