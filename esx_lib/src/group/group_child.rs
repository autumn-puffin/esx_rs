use std::cmp::Ordering;

use crate::{types::FormID, Group, Record, Result};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use super::GroupLabel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum GroupChild {
  Record(Record),
  Group(Group),
  Raw(Bytes),
}

/// Conversion
impl GroupChild {
  pub fn as_bytes(&self) -> Bytes {
    match self {
      Self::Record(r) => r.as_bytes(),
      Self::Group(g) => g.as_bytes(),
      Self::Raw(b) => b.clone(),
    }
  }
  pub fn from_bytes(buf: &mut BytesMut) -> Result<Vec<Self>> {
    let mut children: Vec<Self> = vec![];

    while !buf.is_empty() {
      match &buf[0..4] {
        b"GRUP" => children.push(Self::Group(Group::from_bytes(buf)?)),
        _ => children.push(Self::Record(Record::from_bytes(buf)?)),
      }
    }

    Ok(children)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChildKey {
  Record(FormID),
  Group(GroupLabel),
  Raw,
}

impl From<Record> for ChildKey {
  fn from(record: Record) -> Self {
    Self::Record(*record.get_form_id())
  }
}
impl From<Group> for ChildKey {
  fn from(group: Group) -> Self {
    Self::Group(*group.get_label())
  }
}
impl From<GroupChild> for ChildKey {
  fn from(child: GroupChild) -> Self {
    match child {
      GroupChild::Record(r) => r.into(),
      GroupChild::Group(g) => g.into(),
      GroupChild::Raw(_) => Self::Raw,
    }
  }
}

impl Ord for ChildKey {
  fn cmp(&self, other: &Self) -> Ordering {
    let record_group_cmp = |record: &FormID, group: &GroupLabel| -> Ordering {
      use GroupLabel::*;
      match group {
        Top(_)
        | InteriorCellBlock(_)
        | InteriorCellSubBlock(_)
        | ExteriorCellBlock { .. }
        | ExteriorCellSubBlock { .. } => Ordering::Greater,

        WorldChildren(b)
        | CellChildren(b)
        | TopicChildren(b)
        | CellPersistentChildren(b)
        | CellTemporaryChildren(b)
        | QuestScene(b) => {
          let cmp = record.cmp(b);
          if cmp.is_eq() {
            Ordering::Less
          } else {
            cmp
          }
        }

        Raw { .. } => Ordering::Less,
      }
    };

    match (self, other) {
      (Self::Record(a), Self::Record(b)) => a.cmp(b), // we assume that in a Group, there are no duplicate FormIDs
      (Self::Group(a), Self::Group(b)) => a.cmp(b),

      (Self::Record(a), Self::Group(b)) => record_group_cmp(a, b),
      (Self::Group(a), Self::Record(b)) => record_group_cmp(b, a).reverse(),

      (Self::Raw, Self::Raw) => Ordering::Equal,
      (_, Self::Raw) => Ordering::Less,
      (Self::Raw, _) => Ordering::Greater,
    }
  }
}
impl PartialOrd for ChildKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
