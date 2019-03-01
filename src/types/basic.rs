use std::clone::Clone;

pub type Difficulty = u64;

pub struct Hash {
  data: [u8; 32],
}

#[derive(Clone)]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

pub struct KeyImage {
  pub data: [u8; 32],
}

impl Default for Hash {
  fn default() -> Self {
    Hash { data: [0; 32] }
  }
}

impl Default for Version {
  fn default() -> Self {
    Version {
      major: 1,
      minor: 0,
      patch: 0,
    }
  }
}
