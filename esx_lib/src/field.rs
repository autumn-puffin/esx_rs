use crate::{signature::Signature, Error, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum FieldData {
  Empty,
  Raw(Bytes),
}
impl FieldData {
  fn to_bytes(&self) -> Bytes {
    match self {
      FieldData::Empty => Bytes::new(),
      FieldData::Raw(b) => b.clone(),
    }
  }

  pub fn len(&self) -> usize {
    match self {
      FieldData::Empty => 0,
      FieldData::Raw(b) => b.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      FieldData::Empty => true,
      FieldData::Raw(b) => b.is_empty(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Field {
  signature: Signature,
  data: FieldData,
}

/// Constants
impl Field {
  pub const HEADER_SIZE: usize = 6;
  pub const MAXIMUM_SIZE: usize = Self::HEADER_SIZE + u16::MAX as usize;
}
/// Conversion
impl Field {
  pub fn from_bytes(buf: &mut BytesMut) -> Result<Self> {
    if buf.len() < 6 {
      return Err(Error::BufferTooShort);
    }
    let signature: Signature = Signature::new(&buf.get_u32_le().to_le_bytes());
    let data_size: u16 = buf.get_u16_le();

    if signature == Signature::new(b"XXXX") {
      let size: u32 = buf.get_u32_le();
      return Field::from_oversized_field(buf, size);
    }
    if buf.len() < data_size as usize {
      return Err(Error::BufferTooShort);
    }

    let data: Bytes = buf.split_to(data_size as usize).clone().freeze();

    Ok(Field {
      signature,
      data: FieldData::Raw(data),
    })
  }
  fn from_oversized_field(buf: &mut BytesMut, size: u32) -> Result<Self> {
    if buf.len() < 6 {
      return Err(Error::BufferTooShort);
    }
    let signature: Signature = Signature::new(&buf.get_u32_le().to_le_bytes());
    let _: u16 = buf.get_u16_le();

    if buf.len() < size as usize {
      return Err(Error::BufferTooShort);
    }
    let data: Bytes = buf.split_to(size as usize).clone().freeze();
    Ok(Field {
      signature,
      data: FieldData::Raw(data),
    })
  }

  pub fn as_bytes(&self) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    let data = self.data.to_bytes();
    let data_len = data.len();

    // If oversize
    if data_len > u16::MAX as usize {
      // Preface with oversize field
      bytes.put(Signature::new(b"XXXX").as_bytes());
      bytes.put_u16_le(4);
      bytes.put_u32_le(data_len as u32);

      bytes.put(self.signature.as_slice());
      bytes.put_u16_le(0);
    } else {
      bytes.put(self.signature.as_slice());
      bytes.put_u16_le(data_len as u16);
    }

    bytes.put(data);
    bytes.freeze()
  }
}
/// Getters
impl Field {
  pub fn get_signature(&self) -> &Signature {
    &self.signature
  }
  pub fn get_data(&self) -> &FieldData {
    &self.data
  }
}
/// Process
impl Field {
  pub fn foo() -> Result<()> {
    Ok(())
  }
}
