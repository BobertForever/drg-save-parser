use crate::properties::Property;
use crate::utils::error::ParseError;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Read};

#[derive(Debug, Serialize)]
pub struct IntProperty(pub i32);

impl IntProperty {
  pub fn new(reader: &mut Cursor<Vec<u8>>) -> Result<Property, ParseError> {
    reader.read_exact(&mut [0u8; 1])?;
    Ok(Property::from(IntProperty(
      reader.read_i32::<LittleEndian>()?,
    )))
  }
}
