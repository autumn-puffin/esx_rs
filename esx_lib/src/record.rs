use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::Decompress;
use serde::{Deserialize, Serialize};

use crate::{
  field::Field, form_id::FormID, signature::Signature, timestamp::Timestamp, vcs_info::VcsInfo,
  Error, Result,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum RecordData {
  Empty,
  Raw(BytesMut),
  Compressed(BytesMut),
  Generic(Vec<Field>),
  //Structured(Vec<StructuredField>),
}
/// Conversion
impl RecordData {
  pub fn as_bytes(&self) -> Bytes {
    match self {
      RecordData::Empty => Bytes::new(),
      RecordData::Raw(v) | RecordData::Compressed(v) => v.clone().freeze(),
      RecordData::Generic(f) => {
        let mut bytes: BytesMut = BytesMut::new();
        for field in f {
          bytes.put(field.as_bytes())
        }
        bytes.freeze()
      } // RecordData::Structured(f) => {
        //   let mut vec: Vec<u8> = vec![];
        //   for field in f.to_generic_fields() {
        //     vec.append(&mut field.to_bytes())
        //   }
        //   vec
        // }
    }
  }

  fn generic_from_bytes(buf: &mut BytesMut) -> Result<Self> {
    let mut fields: Vec<Field> = vec![];
    while !buf.is_empty() {
      let field = Field::from_bytes(buf)?;
      fields.push(field);
    }
    Ok(Self::Generic(fields))
  }
  fn generic_from_zlib_bytes(buf: &mut BytesMut) -> Result<Self> {
    let mut d = Decompress::new(true);
    let mut output: BytesMut = BytesMut::new();
    let mut outvec: Vec<u8> = vec![];
    let len = buf.get_u32_le();
    outvec.reserve(len as usize);
    d.decompress_vec(buf, &mut outvec, flate2::FlushDecompress::Finish)?;

    output.put(outvec.as_slice());
    let fields = Self::generic_from_bytes(&mut output)?;
    Ok(fields)
  }
}
/// Getters
impl RecordData {
  pub fn get_fields(&self) -> Vec<&Field> {
    match self {
      RecordData::Empty | RecordData::Raw(_) | RecordData::Compressed(_) => vec![],
      RecordData::Generic(f) => f.iter().collect(),
    }
  }
}
/// Process
impl RecordData {
  pub fn process(&self) -> Result<Self> {
    match self {
      RecordData::Raw(b) => Self::generic_from_bytes(&mut b.clone()),
      RecordData::Compressed(b) => Self::generic_from_zlib_bytes(&mut b.clone()),
      RecordData::Generic(_) => Ok(self.clone()),
      RecordData::Empty => Ok(RecordData::Empty),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Record {
  signature: Signature,
  raw_flags: u32,
  form_id: FormID,
  timestamp: Timestamp,
  vcs_info: VcsInfo,
  form_version: u16,
  _unknown_1: u16,
  data: RecordData,
}
/// Constants
impl Record {
  pub const HEADER_SIZE: usize = 24;
  pub const MAXIMUM_SIZE: usize = Self::HEADER_SIZE + u32::MAX as usize;
}
/// Conversion
impl Record {
  pub fn from_bytes(buf: &mut BytesMut) -> Result<Self> {
    let header: BytesMut = buf.split_to(24);
    let data_size: u32 = u32::from_le_bytes(header[4..8].try_into()?);
    let flags: u32 = u32::from_le_bytes(header[8..12].try_into()?);

    if buf.len() < data_size as usize {
      return Err(Error::BufferTooShort);
    }

    let data: BytesMut = buf.split_to(data_size as usize);
    let data = match (flags & 0x00040000) != 0 {
      true => RecordData::Compressed(data),
      false => RecordData::Raw(data),
    };

    Ok(Record {
      signature: Signature::new(header[0..4].try_into()?),
      raw_flags: flags,
      form_id: u32::from_le_bytes(header[12..16].try_into()?).into(),
      timestamp: u16::from_le_bytes(header[16..18].try_into()?).into(),
      vcs_info: u16::from_le_bytes(header[18..20].try_into()?).into(),
      form_version: u16::from_le_bytes(header[20..22].try_into()?),
      _unknown_1: u16::from_le_bytes(header[22..24].try_into()?),
      data,
    })
  }

  pub fn as_bytes(&self) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    let data = self.data.as_bytes();
    let data_len = data.len();

    bytes.put(self.signature.as_bytes());
    bytes.put_u32_le(data_len as u32);
    bytes.put_u32_le(self.raw_flags);
    bytes.put_u32_le(self.form_id.into());
    bytes.put_u16_le(self.timestamp.into());
    bytes.put_u16_le(self.vcs_info.into());
    bytes.put_u16_le(self.form_version);
    bytes.put_u16_le(self._unknown_1);
    bytes.put(data);

    bytes.freeze()
  }
}
/// Getters
impl Record {
  pub fn get_signature(&self) -> &Signature {
    &self.signature
  }
  pub fn get_form_id(&self) -> &FormID {
    &self.form_id
  }
  pub fn get_form_version(&self) -> &u16 {
    &self.form_version
  }
  pub fn get_data(&self) -> &RecordData {
    &self.data
  }
}
/// Process
impl Record {
  pub fn process(&mut self) {
    match self.data.process() {
      Ok(data) => self.data = data,
      Err(e) => eprintln!("Error processing record data: {:?}", e),
    }
  }
}
