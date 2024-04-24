use std::fmt::{Debug, Display};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FormID(u32);

/// Constants
impl FormID {
  pub const MAX_IDS: u32 = 0x00FFFFFF;
  pub const MAX_IDS_ESL: u32 = 0x00000FFF;
}
/// Conversion
impl FormID {
  pub fn from_bytes(buf: &mut BytesMut) -> Self {
    Self(buf.get_u32_le())
  }
  pub fn as_bytes(&self) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    bytes.put_u32_le(self.0);
    bytes.freeze()
  }
}

impl Debug for FormID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:08x}", self.0)
  }
}
impl Display for FormID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:08x}", self.0)
  }
}

impl From<u32> for FormID {
  fn from(val: u32) -> Self {
    Self(val)
  }
}
impl From<FormID> for u32 {
  fn from(val: FormID) -> Self {
    val.0
  }
}
