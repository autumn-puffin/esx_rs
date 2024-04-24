use std::fmt::{Display, Formatter, Result as fmtResult};

use crate::{Error, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

mod group_data;
mod group_type;
pub use group_data::GroupData;
pub use group_type::GroupLabel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Group {
  label: GroupLabel,
  raw_timestamp: u16,
  raw_vcs_info: u16,
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
      raw_timestamp: header.get_u16_le(),
      raw_vcs_info: header.get_u16_le(),
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
    bytes.put_u16_le(self.raw_timestamp);
    bytes.put_u16_le(self.raw_vcs_info);
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
  pub fn raw_timestamp(&self) -> &u16 {
    &self.raw_timestamp
  }
  pub fn raw_vcs_info(&self) -> &u16 {
    &self.raw_vcs_info
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

impl Display for Group {
  fn fmt(&self, f: &mut Formatter) -> fmtResult {
    write!(
      f,
      "Group:\n\tType: {}\n\tTimestamp: {}\n\tVCS Info: {}\n\tData: {}",
      self.label, self.raw_timestamp, self.raw_vcs_info, self.data
    )
  }
}
