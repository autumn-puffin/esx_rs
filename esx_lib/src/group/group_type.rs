use std::fmt::{Display, Formatter, Result as fmtResult};

use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use crate::{
  types::{FormID, Signature},
  Error, Result,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum GroupLabel {
  Top(Signature),                          // 0 // Record type
  WorldChildren(FormID),                   // 1 // Parent worldspace
  InteriorCellBlock(i32),                  // 2 // Block Number
  InteriorCellSubBlock(i32),               // 3 // Sub-Block Number
  ExteriorCellBlock { x: i16, y: i16 },    // 4 // Coordinates
  ExteriorCellSubBlock { x: i16, y: i16 }, // 5 // Coordinates
  CellChildren(FormID),                    // 6 // Parent cell
  TopicChildren(FormID),                   // 7 // Parent dialog topic
  CellPersistentChildren(FormID),          // 8 // Parent cell
  CellTemporaryChildren(FormID),           // 9 // Parent cell
  QuestScene(FormID),                      // 10 // Parent quest
  Raw { label: [u8; 4], label_type: u32 },
}
/// Conversion
impl GroupLabel {
  pub fn as_bytes(&self) -> Bytes {
    return match self {
      GroupLabel::Raw { label, label_type } => {
        let mut b: BytesMut = BytesMut::new();
        b.put(label.as_slice());
        b.put_u32_le(*label_type);

        b.freeze()
      }
      GroupLabel::Top(s) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(s.as_bytes());
        bytes.put_u32_le(0);

        return bytes.freeze();
      }
      GroupLabel::WorldChildren(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(1);

        return bytes.freeze();
      }
      GroupLabel::InteriorCellBlock(block) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put_i32_le(*block);
        bytes.put_u32_le(2);

        return bytes.freeze();
      }
      GroupLabel::InteriorCellSubBlock(sub_block) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put_i32_le(*sub_block);
        bytes.put_u32_le(3);

        return bytes.freeze();
      }
      GroupLabel::ExteriorCellBlock { x, y } => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put_i16_le(*y);
        bytes.put_i16_le(*x);
        bytes.put_u32_le(4);

        return bytes.freeze();
      }
      GroupLabel::ExteriorCellSubBlock { x, y } => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put_i16_le(*y);
        bytes.put_i16_le(*x);
        bytes.put_u32_le(5);

        return bytes.freeze();
      }
      GroupLabel::CellChildren(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(6);

        return bytes.freeze();
      }
      GroupLabel::TopicChildren(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(7);

        return bytes.freeze();
      }
      GroupLabel::CellPersistentChildren(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(8);

        return bytes.freeze();
      }
      GroupLabel::CellTemporaryChildren(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(9);

        return bytes.freeze();
      }
      GroupLabel::QuestScene(id) => {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.put(id.as_bytes());
        bytes.put_u32_le(10);

        return bytes.freeze();
      }
    };
  }
}
/// Process
impl GroupLabel {
  pub fn process(&self) -> Result<Self> {
    return match self {
      GroupLabel::Raw { label, label_type } => match label_type {
        0 => Ok(GroupLabel::Top(Signature::new(label))),
        1 => Ok(GroupLabel::WorldChildren(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        2 => Ok(GroupLabel::InteriorCellBlock(i32::from_le_bytes(*label))),
        3 => Ok(GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(*label))),
        4 => Ok(GroupLabel::ExteriorCellBlock {
          y: i16::from_le_bytes(label[0..2].try_into()?),
          x: i16::from_le_bytes(label[2..4].try_into()?),
        }),
        5 => Ok(GroupLabel::ExteriorCellSubBlock {
          y: i16::from_le_bytes(label[0..2].try_into()?),
          x: i16::from_le_bytes(label[2..4].try_into()?),
        }),
        6 => Ok(GroupLabel::CellChildren(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        7 => Ok(GroupLabel::TopicChildren(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        8 => Ok(GroupLabel::CellPersistentChildren(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        9 => Ok(GroupLabel::CellTemporaryChildren(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        10 => Ok(GroupLabel::QuestScene(FormID::from_bytes(
          &mut BytesMut::from(label.as_slice()),
        ))),
        _ => Err(Error::UnknownGroupLabelType(*label_type)),
      },
      _ => Ok(self.clone()),
    };
  }
}

impl Display for GroupLabel {
  fn fmt(&self, f: &mut Formatter) -> fmtResult {
    match self {
      GroupLabel::Raw { label, label_type } => {
        write!(f, "Raw: {:?} {}", label, label_type)
      }
      GroupLabel::Top(s) => {
        write!(f, "Top ({})", s)
      }
      GroupLabel::WorldChildren(id) => {
        write!(f, "World Children (WRLD: {})", id)
      }
      GroupLabel::InteriorCellBlock(block) => {
        write!(f, "Interior Cell Block ({})", block)
      }
      GroupLabel::InteriorCellSubBlock(sub_block) => {
        write!(f, "Interior Cell Sub-Block ({})", sub_block)
      }
      GroupLabel::ExteriorCellBlock { x, y } => {
        write!(f, "Exterior Cell Block (X: {}, Y: {})", x, y)
      }
      GroupLabel::ExteriorCellSubBlock { x, y } => {
        write!(f, "Exterior Cell Sub-Block (X: {}, Y: {})", x, y)
      }
      GroupLabel::CellChildren(id) => {
        write!(f, "Cell Children (CELL: {})", id)
      }
      GroupLabel::TopicChildren(id) => {
        write!(f, "Topic Children (DIAL: {})", id)
      }
      GroupLabel::CellPersistentChildren(id) => {
        write!(f, "Cell Persistent Children (CELL: {})", id)
      }
      GroupLabel::CellTemporaryChildren(id) => {
        write!(f, "Cell Temporary Children (CELL: {})", id)
      }
      GroupLabel::QuestScene(id) => {
        write!(f, "Quest Scene (QUST: {})", id)
      }
    }
  }
}
