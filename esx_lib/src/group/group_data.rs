use std::fmt::{Display, Formatter, Result as fmtResult};

use super::Group;
use crate::{record::Record, Result};
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum GroupData {
  Empty,
  Raw(BytesMut),
  Structured(Vec<GroupDataComponent>),
}
/// Conversion
impl GroupData {
  pub fn as_bytes(&self) -> Bytes {
    match self {
      Self::Empty => Bytes::new(),
      Self::Raw(b) => b.clone().freeze(),
      Self::Structured(s) => Self::structure_as_bytes(s),
    }
  }

  fn structure_as_bytes(s: &Vec<GroupDataComponent>) -> Bytes {
    let mut bytes: BytesMut = BytesMut::new();
    for component in s {
      bytes.put(component.as_bytes())
    }
    bytes.freeze()
  }
  fn structure_from_bytes(buf: &mut BytesMut) -> Result<Vec<GroupDataComponent>> {
    let mut components: Vec<GroupDataComponent> = vec![];
    while !buf.is_empty() {
      let component = GroupDataComponent::from_bytes(buf)?;
      components.push(component);
    }
    Ok(components)
  }
}
/// Getters
impl GroupData {
  pub fn get_components(&self) -> Option<&Vec<GroupDataComponent>> {
    match self {
      Self::Empty | Self::Raw(_) => None,
      Self::Structured(s) => Some(s),
    }
  }
  pub fn get_records(&self) -> Vec<&Record> {
    let mut records: Vec<&Record> = vec![];
    match self {
      Self::Empty | Self::Raw(_) => {}
      Self::Structured(s) => {
        records.reserve_exact(s.len());
        for component in s {
          if let GroupDataComponent::Record(r) = component {
            records.push(r)
          }
        }
      }
    };
    records.shrink_to_fit();
    records
  }
  pub fn get_records_recurse(&self) -> Vec<&Record> {
    let mut records: Vec<&Record> = vec![];
    let mut groups_expanded: Vec<Vec<&Record>> = vec![];
    match self {
      Self::Empty | Self::Raw(_) => {}
      Self::Structured(s) => {
        records.reserve_exact(s.len());
        groups_expanded.reserve_exact(s.len());
        for component in s {
          match component {
            GroupDataComponent::Record(r) => records.push(r),
            GroupDataComponent::Group(g) => records.append(&mut g.get_data().get_records_recurse()),
            _ => {}
          }
        }
      }
    };
    let mut add_cap = 0;
    for g in &groups_expanded {
      add_cap += g.len();
    }
    records.reserve_exact(add_cap);
    for g in groups_expanded {
      records.append(&mut g.clone());
    }
    records.shrink_to_fit();
    records
  }
  pub fn get_subgroups(&self) -> Vec<&Group> {
    let mut groups: Vec<&Group> = vec![];
    match self {
      Self::Empty | Self::Raw(_) => {}
      Self::Structured(s) => {
        groups.reserve_exact(s.len());
        for component in s {
          if let GroupDataComponent::Group(g) = component {
            groups.push(g)
          }
        }
      }
    };
    groups.shrink_to_fit();
    groups
  }
}

/// Process
impl GroupData {
  pub fn process(&self) -> Result<Self> {
    match self {
      Self::Raw(b) => {
        let mut structured = GroupData::structure_from_bytes(&mut b.clone())?;
        for component in &mut structured {
          component.process();
        }
        Ok(Self::Structured(structured))
      }
      Self::Structured(s) => {
        let mut s = s.clone();
        for component in &mut s {
          component.process();
        }
        Ok(self.clone())
      }
      Self::Empty => Ok(Self::Empty),
    }
  }
}

impl Display for GroupData {
  fn fmt(&self, f: &mut Formatter) -> fmtResult {
    match self {
      Self::Empty => write!(f, "Empty"),
      Self::Raw(b) => write!(f, "Raw Length: {}", b.len()),
      Self::Structured(s) => write!(f, "Structured: {:?}", s),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum GroupDataComponent {
  Empty,
  Group(Group),
  Record(Record),
}
/// Conversion
impl GroupDataComponent {
  fn as_bytes(&self) -> Bytes {
    match self {
      Self::Empty => Bytes::new(),
      Self::Group(g) => g.as_bytes(),
      Self::Record(r) => r.as_bytes(),
    }
  }

  fn from_bytes(buf: &mut BytesMut) -> Result<GroupDataComponent> {
    match &buf[0..4] {
      b"GRUP" => Ok(Group::from_bytes(buf)?.into()),
      _ => Ok(Record::from_bytes(buf)?.into()),
    }
  }

  fn process(&mut self) {
    match self {
      Self::Group(g) => g.process(),
      Self::Record(r) => r.process(),
      _ => {}
    }
  }
}

impl From<Group> for GroupDataComponent {
  fn from(g: Group) -> Self {
    Self::Group(g)
  }
}
impl From<Record> for GroupDataComponent {
  fn from(r: Record) -> Self {
    Self::Record(r)
  }
}
