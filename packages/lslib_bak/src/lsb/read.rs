use std::{
  collections::HashMap,
  io::{Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
  error::Error,
  resource::{
    metadata::{read::ResourceMetadataReadExt, Metadata},
    node::{
      attribute::Attribute,
      data::{Data, Kind},
    },
    value::{read::ResourceValueReadExt, Value},
    Resource,
  },
  util::{
    arena::{Arena, ArenaWriter, Index},
    read::BinaryReadExt,
  },
};

use super::header::{SIGNATURE_BG3, SIGNATURE_FW3};

pub trait LsbReadExt: Read + Seek {
  fn read_lsb_resource(&mut self) -> Result<Resource, Error> {
    let signature: [u8; 4] = self.read_u32::<LittleEndian>()?.to_be_bytes();

    if signature != SIGNATURE_BG3 && signature != SIGNATURE_FW3 {
      return Err(Error::InvalidSignature(SIGNATURE_BG3, signature));
    }

    let is_bg3 = signature == SIGNATURE_BG3;

    let _total_size = self.read_u32::<LittleEndian>()?;
    let _big_endian = self.read_u32::<LittleEndian>()?;
    let _unknown = self.read_u32::<LittleEndian>()?;
    let mut metadata = Metadata::new();
    self.read_resource_metadata(&mut metadata)?;

    let string_count = self.read_u32::<LittleEndian>()?;
    let mut strings: HashMap<u32, String> = HashMap::new();
    for _ in 0..string_count {
      let string_length = self.read_i32::<LittleEndian>()?;
      let string = self.read_utf8_string(string_length as usize)?;
      let index = self.read_u32::<LittleEndian>()?;
      strings.insert(index, string);
    }

    let region_count = self.read_u32::<LittleEndian>()?;
    let mut arena: Arena<Data> = Arena::new();
    for _ in 0..region_count {
      let region_name_id = self.read_u32::<LittleEndian>()?;
      let region_offset = self.read_u32::<LittleEndian>()?;
      let region_name = strings.get(&region_name_id).unwrap();
      let last_region_position = self.stream_position()?;
      self.seek(SeekFrom::Start(region_offset as u64))?;
      self.read_lsb_node(
        &mut arena,
        &strings,
        is_bg3,
        Kind::Region(region_name.to_owned()),
        None,
      )?;
      self.seek(SeekFrom::Start(last_region_position))?;
    }

    Ok(Resource::new_with_arena(arena))
  }

  fn read_lsb_node(
    &mut self,
    arena: &mut Arena<Data>,
    strings: &HashMap<u32, String>,
    is_bg3: bool,
    node_kind: Kind,
    parent_index: Option<Index>,
  ) -> Result<(), Error> {
    let node_name_id = self.read_u32::<LittleEndian>()?;
    let attribute_count = self.read_u32::<LittleEndian>()?;
    let child_count = self.read_u32::<LittleEndian>()?;
    let node_name = strings.get(&node_name_id).unwrap().to_owned();
    let mut node_data = Data::new(node_name);
    node_data.set_kind(node_kind);
    let node_index = arena.alloc(node_data, parent_index);

    for _ in 0..attribute_count {
      let attribute_name_id = self.read_u32::<LittleEndian>()?;
      let attribute_type_id = self.read_u32::<LittleEndian>()?;
      let attribute_value = self.read_lsb_attribute_value(attribute_type_id, is_bg3)?;
      let attribute_name = strings.get(&attribute_name_id).unwrap().to_owned();

      arena
        .value_mut(node_index)
        .attributes_mut()
        .insert(attribute_name, Attribute::new_value(attribute_value));
    }

    for _ in 0..child_count {
      self.read_lsb_node(arena, strings, is_bg3, Kind::Element, Some(node_index))?;
    }

    Ok(())
  }

  fn read_lsb_attribute_value(&mut self, type_id: u32, is_bg3: bool) -> Result<Value, Error> {
    match type_id {
      20 => {
        let string_length = self.read_i32::<LittleEndian>()?;
        Ok(Value::String(
          self.read_utf8_string(string_length as usize)?,
        ))
      }
      21 => {
        let string_length = self.read_i32::<LittleEndian>()?;
        Ok(Value::Path(self.read_utf8_string(string_length as usize)?))
      }
      22 => {
        let string_length = self.read_i32::<LittleEndian>()?;
        Ok(Value::FixedString(
          self.read_utf8_string(string_length as usize)?,
        ))
      }
      23 => {
        let string_length = self.read_i32::<LittleEndian>()?;
        Ok(Value::LsString(
          self.read_utf8_string(string_length as usize)?,
        ))
      }
      25 => {
        let buffer_length = self.read_i32::<LittleEndian>()?;
        Ok(Value::ScratchBuffer(
          self.read_bytes(buffer_length as usize)?,
        ))
      }
      28 => {
        let (version, value) = if is_bg3 {
          let mut version = self.read_u16::<LittleEndian>()?;
          let mut value = "".to_owned();
          let test = self.read_u32::<LittleEndian>()?;
          if test == 0 {
            self.seek(SeekFrom::Current(-4))?;
            version = 0;
            let value_length = self.read_i32::<LittleEndian>()?;
            value = self.read_utf8_string(value_length as usize)?;
          } else {
            self.seek(SeekFrom::Current(-2))?;
          }
          (version, value)
        } else {
          let value_length = self.read_i32::<LittleEndian>()?;
          let value = self.read_utf8_string(value_length as usize)?;
          (0u16, value)
        };
        let handle_length = self.read_i32::<LittleEndian>()?;
        let handle = self.read_utf8_string(handle_length as usize)?;
        Ok(Value::TranslatedString {
          version,
          value,
          handle,
        })
      }
      29 => {
        let string_length = self.read_i32::<LittleEndian>()? * 2;
        Ok(Value::WString(
          self.read_utf8_string(string_length as usize)?,
        ))
      }
      30 => {
        let string_length = self.read_i32::<LittleEndian>()? * 2;
        Ok(Value::LswString(
          self.read_utf8_string(string_length as usize)?,
        ))
      }
      _ => self.read_resource_value(type_id),
    }
  }
}

impl<R: Read + Seek + ?Sized> LsbReadExt for R {}
