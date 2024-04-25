use std::fmt::Display;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature([u8; 4]);

impl PartialOrd for Signature {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Signature {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.as_string().cmp(&other.as_string())
  }
}

/// Conversion
impl Signature {
  pub fn new(sig: &[u8; 4]) -> Self {
    Signature(*sig)
  }
  pub fn from_bytes(bytes: Bytes) -> Self {
    let mut sig = [0; 4];
    sig.copy_from_slice(&bytes);
    Signature(sig)
  }
}
/// Getters
impl Signature {
  pub fn as_bytes(&self) -> Bytes {
    (*self).into()
  }
  pub fn as_string(&self) -> String {
    (*self).into()
  }
  pub fn as_slice(&self) -> &[u8] {
    self.0.as_slice()
  }
}

impl std::fmt::Debug for Signature {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}
impl Display for Signature {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}
impl From<Signature> for Bytes {
  fn from(val: Signature) -> Self {
    Bytes::copy_from_slice(val.as_slice())
  }
}
impl From<Signature> for String {
  fn from(val: Signature) -> Self {
    // deal with _IAD signatures
    if &val.0[1..=3] == b"IAD" {
      let imad_val = val.0[0];
      return format!("({:02x?})IAD", imad_val);
    }

    String::from_utf8_lossy(&val.0).into_owned()
  }
}
