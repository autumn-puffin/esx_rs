//! A library for reading and writing ESM, ESP, and ESL files for The Elder Scrolls IV and later games.
//!
//! # Supported Games
//! - The Elder Scrolls IV: Oblivion
//! - The Elder Scrolls V: Skyrim
//! - Fallout 3
//! - Fallout: New Vegas
//! - Fallout 4
//! - Fallout 76
//! - Starfield
//!
//! > Note: This library is not yet complete and may not recognize all records and fields in all games.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub use error::{Error, Result};

pub mod esx;
pub mod field;
pub mod group;
pub mod record;
pub mod types;
