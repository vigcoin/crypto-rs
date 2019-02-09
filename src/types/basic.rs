pub type Difficulty = u64;

pub struct Hash {
  data: [u8; 32],
}

pub struct Version {
  major: u8,
  minor: u8,
  patch: u8,
}

pub struct KeyImage {
  data: [u8; 32],
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
