use crate::{
  types::{Timestamp, VcsInfo},
  Error, Result,
};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

mod group_data;
mod group_type;
pub use group_data::GroupData;
pub use group_type::GroupLabel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Group {
  label: GroupLabel,
  timestamp: Timestamp,
  vcs_info: VcsInfo,
  _unknown_1: u32,
  data: GroupData,
}
/// Constants
impl Group {
  pub const HEADER_SIZE: usize = 24;
  pub const MAXIMUM_SIZE: usize = u32::MAX as usize;
}
/// Conversion
impl Group {
  pub fn from_bytes(buf: &mut BytesMut) -> Result<Self> {
    if &buf[0..4] != b"GRUP".as_slice() {
      return Err(Error::NonGroupSignature(buf[0..4].to_vec()));
    }

    let mut header: BytesMut = buf.split_to(24);
    let _signature = header.split_to(4);
    let data_size: u32 = header.get_u32_le();

    if buf.len() < (data_size as usize - 24) {
      return Err(Error::BufferTooShort);
    };

    let data: BytesMut = buf.split_to(data_size as usize - 24);

    let mut label: [u8; 4] = [0; 4];
    header.copy_to_slice(&mut label);

    Ok(Self {
      label: GroupLabel::Raw {
        label,
        label_type: header.get_u32_le(),
      },
      timestamp: header.get_u16_le().into(),
      vcs_info: header.get_u16_le().into(),
      _unknown_1: header.get_u32_le(),
      data: GroupData::Raw(data),
    })
  }

  pub fn as_bytes(&self) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    let data = self.data.as_bytes();
    let data_len = data.len() + Self::HEADER_SIZE;

    bytes.put(b"GRUP".as_slice());
    bytes.put_u32_le(data_len as u32);
    bytes.put(self.label.as_bytes());
    bytes.put_u16_le(self.timestamp.into());
    bytes.put_u16_le(self.vcs_info.into());
    bytes.put_u32_le(self._unknown_1);
    bytes.put(data);

    bytes.freeze()
  }
}
/// Getters
impl Group {
  pub fn get_label(&self) -> &GroupLabel {
    &self.label
  }
  pub fn get_timestamp(&self) -> &Timestamp {
    &self.timestamp
  }
  pub fn get_vcs_info(&self) -> &VcsInfo {
    &self.vcs_info
  }
  pub fn get_data(&self) -> &GroupData {
    &self.data
  }
}
/// Processing
impl Group {
  pub fn process(&mut self) {
    self.process_label();
    self.process_data();
  }
  pub fn process_label(&mut self) {
    match self.label.process() {
      Ok(l) => self.label = l,
      Err(e) => eprintln!("Error processing Group Label: {:?}", e),
    }
  }
  pub fn process_data(&mut self) {
    match self.data.process() {
      Ok(d) => self.data = d,
      Err(e) => eprintln!("Error processing Group Data: {:?}", e),
    }
  }
}

impl Ord for Group {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.label.cmp(&other.label)
  }
}
impl PartialOrd for Group {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
