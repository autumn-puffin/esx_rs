use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct VcsInfo {
  last_user: u8,
  current_user: u8,
}

impl VcsInfo {
  pub fn from_bytes(buf: &mut BytesMut) -> Self {
    Self {
      last_user: buf.get_u8(),
      current_user: buf.get_u8(),
    }
  }
  pub fn as_bytes(&self) -> BytesMut {
    let mut bytes = BytesMut::new();
    bytes.put_u8(self.last_user);
    bytes.put_u8(self.current_user);
    bytes
  }
}

impl From<u16> for VcsInfo {
  fn from(val: u16) -> Self {
    Self {
      last_user: ((val & 0x00FF) >> 8) as u8,
      current_user: (val & 0xFF00) as u8,
    }
  }
}
impl From<VcsInfo> for u16 {
  fn from(val: VcsInfo) -> u16 {
    let last_user = ((val.last_user as u16) << 8) & 0x00FF;
    let current_user = val.current_user as u16 & 0xFF00;
    last_user + current_user
  }
}
