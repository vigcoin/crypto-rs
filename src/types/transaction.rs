use super::basic::*;

use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;

pub struct BaseInput {
  blockHeight: u32,
}

pub struct KeyInput {
  amount: u64,
  outputs: Vec<u32>,
  keyImage: KeyImage,
}

pub struct MultiSignatureInput {
  amount: u64,
  signatureCount: u8,
  output: u32,
}

pub enum Input {
  base(BaseInput),
  key(KeyInput),
  signature(MultiSignatureInput),
}

pub struct KeyOutput {
  key: PublicKey,
}

pub struct MultiSignatureOutput {
  keys: Vec<PublicKey>,
  requiredSignatureCount: u8,
}

pub enum BaseOutput {
  key(KeyOutput),
  signature(MultiSignatureOutput),
}

pub struct Output {
  amount: u64,
  base: BaseOutput,
}

pub struct Transaction {
  version: u8,
  unlockTime: u64,
  inputs: Vec<Input>,
  outputs: Vec<Output>,
  extra: Vec<u8>,
  signatures: Vec<Vec<Signature>>,
}

pub struct TransactionEntry {
  tx: Transaction,
  indexes: Vec<u32>,
}
