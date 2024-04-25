use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Timestamp {
  year: u8,
  month: u8,
  day: u8,
}
/// Constants
impl Timestamp {
  pub const YEAR_MASK: u16 = 0b1111111000000000;
  pub const MONTH_MASK: u16 = 0b0000000111100000;
  pub const DAY_MASK: u16 = 0b0000000000011111;
}

impl From<u16> for Timestamp {
  fn from(val: u16) -> Self {
    Self {
      year: ((val & Self::YEAR_MASK) >> 9) as u8,
      month: ((val & Self::MONTH_MASK) >> 5) as u8,
      day: (val & Self::DAY_MASK) as u8,
    }
  }
}
impl From<Timestamp> for u16 {
  fn from(val: Timestamp) -> u16 {
    let year = ((val.year as u16) << 9) & Timestamp::YEAR_MASK;
    let month = ((val.month as u16) << 5) & Timestamp::MONTH_MASK;
    let day = val.day as u16 & Timestamp::DAY_MASK;
    year + month + day
  }
}
