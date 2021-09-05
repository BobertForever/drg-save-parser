use crate::properties::{GuidProperty, Property, StructProperty};
use crate::utils::{error::ParseError, read_string::*};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Read};

#[derive(Debug, Serialize)]
pub struct ArrayProperty(pub Vec<Box<Property>>);

impl ArrayProperty {
  pub fn new(reader: &mut Cursor<Vec<u8>>) -> Result<Property, ParseError> {
    let array_property_type = reader.read_string()?;
    reader.read_exact(&mut [0u8; 1])?;

    let properties = ArrayProperty::parse_property_list(reader, array_property_type)?;
    Ok(Property::from(ArrayProperty(properties)))
  }

  fn parse_property_list(
    reader: &mut Cursor<Vec<u8>>,
    propert_type: String,
  ) -> Result<Vec<Box<Property>>, ParseError> {
    let num_properties = reader.read_i32::<LittleEndian>()?;
    let properties = match propert_type.as_str() {
      "StructProperty" => ArrayProperty::parse_struct_property_list(reader, num_properties)?,
      "ObjectProperty" => {
        let mut properties: Vec<Box<Property>> = Vec::new();
        for _ in 0..num_properties {
          properties.push(Box::new(Property::new("ObjectProperty", reader)?))
        }
        properties
      }
      _ => {
        return Err(ParseError::new(format!(
          "Unhandled array data type {}",
          propert_type
        )))
      }
    };
    Ok(properties)
  }

  fn parse_struct_property_list(
    reader: &mut Cursor<Vec<u8>>,
    num_properties: i32,
  ) -> Result<Vec<Box<Property>>, ParseError> {
    let _name = reader.read_string()?;
    let property_type = reader.read_string()?;
    let _length = reader.read_i64::<LittleEndian>()?;

    let mut properties: Vec<Box<Property>> = Vec::new();
    match property_type.as_str() {
      "StructProperty" => {
        let struct_inner_property_type = reader.read_string()?;
        reader.read_exact(&mut [0u8; 17])?;

        for _ in 0..num_properties {
          let property = match struct_inner_property_type.as_str() {
            "Guid" => GuidProperty::new(reader)?,
            _ => StructProperty::parse_property_array(reader)?,
          };
          properties.push(Box::new(property));
        }
      }
      _ => {
        return Err(ParseError::new(format!(
          "Unhandled Array property type {}",
          property_type
        )))
      }
    }

    Ok(properties)
  }
}
