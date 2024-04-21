use std::{fs::File, io::Read};

use crate::{group::Group, record::Record, Error, Result};
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ESx {
  header_record: Record,
  top_groups: Vec<Group>,
}

/// Conversion
impl ESx {
  pub fn from_file(file: &File) -> Result<Self> {
    let mut file = file;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;

    return Self::from_bytes(&mut BytesMut::from(buf.as_slice()));
  }
  pub fn from_bytes(buf: &mut BytesMut) -> Result<Self> {
    match &buf[0..4] {
      b"TES4" => {}
      b"TES3" => return Err(Error::TES3Header),
      _ => return Err(Error::UnknownFileType),
    };
    let header_record = Record::from_bytes(buf)?;
    let mut top_groups: Vec<Group> = vec![];

    while !buf.is_empty() {
      top_groups.push(Group::from_bytes(buf)?);
    }

    Ok(Self {
      header_record,
      top_groups,
    })
  }

  pub fn as_bytes(&self) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    let header = self.header_record.as_bytes();
    let mut data: BytesMut = BytesMut::new();

    for group in &self.top_groups {
      data.put(group.as_bytes())
    }

    bytes.put(header);
    bytes.put(data);

    bytes.freeze()
  }
}
/// Getters
impl ESx {
  pub fn get_header_record(&self) -> &Record {
    &self.header_record
  }
  pub fn get_top_groups(&self) -> &Vec<Group> {
    &self.top_groups
  }
  pub fn get_all_records(&self) -> Vec<&Record> {
    let mut records: Vec<&Record> = vec![&self.header_record];
    for group in &self.top_groups {
      records.append(&mut group.get_data().get_records_recurse());
    }
    records
  }
}
/// Process
impl ESx {
  pub fn process(&mut self) {
    self.process_header();
    self.process_groups();
  }
  pub fn process_header(&mut self) {
    self.header_record.process();
  }
  pub fn process_groups(&mut self) {
    for group in &mut self.top_groups {
      group.process();
    }
  }
}
