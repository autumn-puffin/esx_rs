use std::collections::BTreeMap;

use esx_lib::field::{Field, FieldData};
use serde::{Deserialize, Serialize};

// Based on a field's data, attempt to determine the type of data it contains.
pub trait FieldFingerprinting {
  fn fingerprint(&self) -> FieldFingerprint;
}

impl FieldFingerprinting for Vec<&Field> {
  fn fingerprint(&self) -> FieldFingerprint {
    let mut data_types: BTreeMap<FieldDataType, u32> = BTreeMap::new();
    for field in self {
      let field_type: FieldDataType = 'field_type: {
        let data = match field.get_data() {
          FieldData::Empty => break 'field_type FieldDataType::Null,
          FieldData::Raw(data) => data.clone(),
        };

        let len = data.len();

        // Check for null data
        if len == 0 {
          break 'field_type FieldDataType::Null;
        }

        let first_u8: Option<u8> = data.get(0).copied();
        let first_u16: Option<u16> = data.get(..2).and_then(|slice| {
          let Ok(b) = slice.try_into() else { return None };
          Some(u16::from_le_bytes(b))
        });

        let is_u8_len_match = match first_u8 {
          Some(u8) => u8 as usize + 1 == len,
          None => false,
        };
        let is_u16_len_match = match first_u16 {
          Some(u16) => u16 as usize + 2 == len,
          None => false,
        };

        let is_last_nullterm = data.last() == Some(&0);

        let is_ascii = 'check_ascii: {
          let nullterm: usize = if is_last_nullterm { 1 } else { 0 };

          let check_for_ascii = match (is_u8_len_match, is_u16_len_match) {
            (false, false) => data.get(..len - nullterm),
            (true, false) => data.get(1..len - nullterm),
            (_, true) => data.get(2..len - nullterm),
          }
          .unwrap_or(&[0]);
          if check_for_ascii.is_empty() {
            break 'check_ascii false;
          }
          check_for_ascii.is_ascii()
        };

        let is_len_match_ascii: bool = match (is_u8_len_match, is_u16_len_match) {
          (false, false) => false,
          (true, false) => data.get(0).unwrap_or(&0).is_ascii(),
          (_, true) => data.get(..2).unwrap_or(&[0]).is_ascii(),
        };

        // Check for BString, BZString, WString, WZString, String, ZString
        if is_ascii {
          match (is_u8_len_match, is_u16_len_match, is_last_nullterm) {
            (true, false, false) => break 'field_type FieldDataType::BString(len),
            (true, false, true) => break 'field_type FieldDataType::BZString(len),
            (_, true, false) => break 'field_type FieldDataType::WString(len),
            (_, true, true) => break 'field_type FieldDataType::WZString(len),
            (false, false, false) => break 'field_type FieldDataType::String(len),
            (false, false, true) => break 'field_type FieldDataType::ZString(len),
          };
        };

        // Otherwise unknown
        break 'field_type FieldDataType::Unknown(len);
      };
      data_types
        .entry(field_type)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    }
    FieldFingerprint { data_types }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FieldDataType {
  Unknown(usize),
  Null,
  Char,
  WChar,
  VariableValue,
  FileTime,
  SystemTime,
  BString(usize),
  BZString(usize),
  WString(usize),
  WZString(usize),
  ZString(usize),
  String(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FieldFingerprint {
  data_types: BTreeMap<FieldDataType, u32>,
}
impl FieldFingerprint {
  pub fn get_data_types(&self) -> &BTreeMap<FieldDataType, u32> {
    &self.data_types
  }
}
