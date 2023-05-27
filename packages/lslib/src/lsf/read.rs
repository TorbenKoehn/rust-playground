use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
  compression::{read::DecompressReadExt, CompressionOptions},
  error::Error,
  resource::{
    node::{attribute::Attribute, data::Data},
    value::{read::ResourceValueReadExt, TranslatedFsStringArgument, Value},
    Resource,
  },
  util::{
    arena::{Arena, ArenaWriter},
    read::BinaryReadExt,
  },
};

use super::{
  attribute::AttributeInfo,
  context::Context,
  header::{Version, SIGNATURE},
  node::NodeInfo,
};

pub trait LsfReadExt: Read + Seek {
  fn read_lsf_resource(&mut self) -> Result<Resource, Error> {
    let mut context = Context::new();
    self.read_lsf_header(&mut context)?;
    self.read_lsf_strings(&mut context)?;
    self.read_lsf_node_infos(&mut context)?;
    self.read_lsf_attribute_infos(&mut context)?;
    let arena = self.read_lsf_node_arena(&mut context)?;
    Ok(Resource::new_with_arena(arena))
  }

  fn read_lsf_header(&mut self, context: &mut Context) -> Result<(), Error> {
    let header = context.header_mut();
    let mut signature = [0u8; 4];
    self.read_exact(&mut signature)?;
    if signature != SIGNATURE {
      return Err(Error::InvalidSignature(signature, SIGNATURE));
    }

    let version: Version = self.read_u32::<LittleEndian>()?.into();
    header.set_version(version);

    if version >= Version::V5 {
      let engine_version = self.read_i64::<LittleEndian>()?;
      header.set_engine_version(engine_version);
    } else {
      let engine_version = self.read_i32::<LittleEndian>()?;
      header.set_engine_version(engine_version as i64);
    }

    if version < Version::V6 {
      let strings_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let strings_size_on_disk = self.read_u32::<LittleEndian>()?;
      let nodes_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let nodes_size_on_disk = self.read_u32::<LittleEndian>()?;
      let attributes_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let attributes_size_on_disk = self.read_u32::<LittleEndian>()?;
      let values_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let values_size_on_disk = self.read_u32::<LittleEndian>()?;
      let compression_flags = self.read_u8()?;
      let _unknown2 = self.read_u8()?;
      let _unknown3 = self.read_u16::<LittleEndian>()?;
      let has_sibling_data = self.read_u32::<LittleEndian>()?;

      header.set_strings_uncompressed_size(strings_uncompressed_size);
      header.set_strings_size_on_disk(strings_size_on_disk);
      header.set_nodes_uncompressed_size(nodes_uncompressed_size);
      header.set_nodes_size_on_disk(nodes_size_on_disk);
      header.set_attributes_uncompressed_size(attributes_uncompressed_size);
      header.set_attributes_size_on_disk(attributes_size_on_disk);
      header.set_values_uncompressed_size(values_uncompressed_size);
      header.set_values_size_on_disk(values_size_on_disk);
      header.set_compression_flags(compression_flags);
      header.set_has_sibling_data(has_sibling_data);
    } else {
      let strings_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let strings_size_on_disk = self.read_u32::<LittleEndian>()?;
      let _unknown1 = self.read_u64::<LittleEndian>()?;
      let nodes_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let nodes_size_on_disk = self.read_u32::<LittleEndian>()?;
      let attributes_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let attributes_size_on_disk = self.read_u32::<LittleEndian>()?;
      let values_uncompressed_size = self.read_u32::<LittleEndian>()?;
      let values_size_on_disk = self.read_u32::<LittleEndian>()?;
      let compression_flags = self.read_u8()?;
      let _unknown2 = self.read_u8()?;
      let _unknown3 = self.read_u16::<LittleEndian>()?;
      let has_sibling_data = self.read_u32::<LittleEndian>()?;

      header.set_strings_uncompressed_size(strings_uncompressed_size);
      header.set_strings_size_on_disk(strings_size_on_disk);
      header.set_nodes_uncompressed_size(nodes_uncompressed_size);
      header.set_nodes_size_on_disk(nodes_size_on_disk);
      header.set_attributes_uncompressed_size(attributes_uncompressed_size);
      header.set_attributes_size_on_disk(attributes_size_on_disk);
      header.set_values_uncompressed_size(values_uncompressed_size);
      header.set_values_size_on_disk(values_size_on_disk);
      header.set_compression_flags(compression_flags);
      header.set_has_sibling_data(has_sibling_data);
    }

    Ok(())
  }

  fn read_lsf_strings(&mut self, context: &mut Context) -> Result<(), Error> {
    let header = context.header();
    let uncompressed_size = header.strings_uncompressed_size();
    let size_on_disk = header.strings_size_on_disk();

    let uncompressed = self.read_decompressed(
      size_on_disk as usize,
      uncompressed_size as usize,
      header.compression_flags().into(),
    )?;

    let mut cursor = Cursor::new(uncompressed);
    let hash_entry_count = cursor.read_u32::<LittleEndian>()?;
    context
      .string_lists_mut()
      .reserve(hash_entry_count as usize);

    for _ in 0..hash_entry_count {
      let string_count = cursor.read_u16::<LittleEndian>()?;
      let mut string_list: Vec<String> = Vec::with_capacity(string_count as usize);
      for _ in 0..string_count {
        let string_length = cursor.read_u16::<LittleEndian>()?;
        let string = cursor.read_utf8_string(string_length as usize)?;
        string_list.push(string);
      }
      context.string_lists_mut().push(string_list);
    }

    Ok(())
  }

  fn read_lsf_node_infos(&mut self, context: &mut Context) -> Result<(), Error> {
    let header = context.header();
    let uncompressed_size = header.nodes_uncompressed_size();
    let size_on_disk = header.nodes_size_on_disk();
    let chunks_allowed = header.version() >= &Version::V2;
    let has_sibling_data = header.version() >= &Version::V3 && header.has_sibling_data() == 1;

    let mut compression_options: CompressionOptions = header.compression_flags().into();
    compression_options.set_chunked(chunks_allowed);
    let uncompressed = self.read_decompressed(
      size_on_disk as usize,
      uncompressed_size as usize,
      compression_options,
    )?;

    let mut cursor = Cursor::new(uncompressed);
    while cursor.position() < uncompressed_size as u64 {
      let mut node = NodeInfo::new();
      if has_sibling_data {
        let name_hash_table_index = cursor.read_u32::<LittleEndian>()?;
        let parent_index = cursor.read_i32::<LittleEndian>()?;
        let _next_sibling_index = cursor.read_i32::<LittleEndian>()?;
        let first_attribute_index = cursor.read_i32::<LittleEndian>()?;
        let name_index = (name_hash_table_index >> 16) as i32;
        let name_offset = (name_hash_table_index & 0xffff) as i32;
        node.set_name_index(name_index);
        node.set_name_offset(name_offset);
        node.set_parent_index(parent_index);
        node.set_first_attribute_index(first_attribute_index);
      } else {
        let name_hash_table_index = cursor.read_u32::<LittleEndian>()?;
        let first_attribute_index = cursor.read_i32::<LittleEndian>()?;
        let parent_index = cursor.read_i32::<LittleEndian>()?;
        let name_index = (name_hash_table_index >> 16) as i32;
        let name_offset = (name_hash_table_index & 0xffff) as i32;
        node.set_name_index(name_index);
        node.set_name_offset(name_offset);
        node.set_first_attribute_index(first_attribute_index);
        node.set_parent_index(parent_index);
      }
      context.nodes_infos_mut().push(node);
    }

    Ok(())
  }

  fn read_lsf_attribute_infos(&mut self, context: &mut Context) -> Result<(), Error> {
    let header = context.header();
    let uncompressed_size = header.attributes_uncompressed_size();
    let size_on_disk = header.attributes_size_on_disk();
    let chunks_allowed = header.version() >= &Version::V2;
    let has_sibling_data = header.version() >= &Version::V3 && header.has_sibling_data() == 1;

    let mut compression_options: CompressionOptions = header.compression_flags().into();
    compression_options.set_chunked(chunks_allowed);
    let uncompressed = self.read_decompressed(
      size_on_disk as usize,
      uncompressed_size as usize,
      compression_options,
    )?;

    let mut cursor = Cursor::new(uncompressed);
    if has_sibling_data {
      while cursor.position() < uncompressed_size as u64 {
        let mut attribute = AttributeInfo::new();
        let name_hash_table_index = cursor.read_u32::<LittleEndian>()?;
        let type_and_length = cursor.read_u32::<LittleEndian>()?;
        let next_attribute_index = cursor.read_i32::<LittleEndian>()?;
        let offset = cursor.read_u32::<LittleEndian>()?;
        let name_index = (name_hash_table_index >> 16) as i32;
        let name_offset = (name_hash_table_index & 0xffff) as i32;
        let type_id = type_and_length & 0x3f;
        let length = type_and_length >> 6;
        attribute.set_name_index(name_index);
        attribute.set_name_offset(name_offset);
        attribute.set_type_id(type_id);
        attribute.set_length(length);
        attribute.set_data_offset(offset);
        attribute.set_next_attribute_index(next_attribute_index);

        context.attribute_infos_mut().push(attribute);
      }
    } else {
      let mut refs: Vec<i32> = Vec::new();
      let mut data_offset = 0u32;
      let mut index = 0i32;
      while cursor.position() < uncompressed_size as u64 {
        let mut attribute_info = AttributeInfo::new();
        let name_hash_table_index = cursor.read_u32::<LittleEndian>()?;
        let type_and_length = cursor.read_u32::<LittleEndian>()?;
        let node_index = cursor.read_i32::<LittleEndian>()?;
        let name_index = (name_hash_table_index >> 16) as i32;
        let name_offset = (name_hash_table_index & 0xffff) as i32;
        let type_id = type_and_length & 0x3f;
        let length = type_and_length >> 6;
        attribute_info.set_name_index(name_index);
        attribute_info.set_name_offset(name_offset);
        attribute_info.set_type_id(type_id);
        attribute_info.set_length(length);
        attribute_info.set_data_offset(data_offset);

        let current_node_index: usize = (node_index + 1) as usize;
        if refs.len() > current_node_index {
          let attr_ref = refs[current_node_index];
          if attr_ref != -1 {
            context
              .attribute_infos_mut()
              .get_mut(attr_ref as usize)
              .unwrap()
              .set_next_attribute_index(index);
          }
          refs[current_node_index] = index;
        } else {
          while refs.len() < current_node_index {
            refs.push(-1);
          }

          refs.push(index);
        }

        data_offset += length;
        context.attribute_infos_mut().push(attribute_info);
        index += 1;
      }
    }

    Ok(())
  }

  fn read_lsf_node_arena(&mut self, context: &mut Context) -> Result<Arena<Data>, Error> {
    let header = context.header();
    let uncompressed_size = header.values_uncompressed_size();
    let size_on_disk = header.values_size_on_disk();
    let chunks_allowed = header.version() >= &Version::V2;

    let mut compression_options: CompressionOptions = header.compression_flags().into();
    compression_options.set_chunked(chunks_allowed);
    let uncompressed = self.read_decompressed(
      size_on_disk as usize,
      uncompressed_size as usize,
      compression_options,
    )?;

    let mut cursor = Cursor::new(uncompressed);
    let mut node_arena: Arena<Data> = Arena::new();
    for node_info in context.node_infos() {
      let node_data = cursor.read_lsf_node_data(node_info, context)?;
      if node_info.parent_index() != -1 {
        let parent_index = node_info.parent_index() as usize;
        node_arena.alloc(node_data, Some(parent_index));
      } else {
        node_arena.alloc(node_data, None);
      };
    }

    Ok(node_arena)
  }

  fn read_lsf_node_data(&mut self, node_info: &NodeInfo, context: &Context) -> Result<Data, Error> {
    let name = context
      .string_lists()
      .get(node_info.name_index() as usize)
      .ok_or(Error::InvalidStringIndex(node_info.name_index()))?
      .get(node_info.name_offset() as usize)
      .ok_or(Error::InvalidStringOffset(
        node_info.name_index(),
        node_info.name_offset(),
      ))?;

    let mut node_data = Data::new(name.to_owned());
    if node_info.first_attribute_index() == -1 {
      return Ok(node_data);
    }

    let mut attribute_info = context
      .attribute_infos()
      .get(node_info.first_attribute_index() as usize)
      .ok_or(Error::InvalidAttributeIndex(
        node_info.first_attribute_index(),
      ))?;
    loop {
      self.seek(SeekFrom::Start(attribute_info.data_offset() as u64))?;
      let attribute = self.read_lsf_attribute(attribute_info, context)?;
      let attribute_name = context
        .string_lists()
        .get(attribute_info.name_index() as usize)
        .ok_or(Error::InvalidStringIndex(attribute_info.name_index()))?
        .get(attribute_info.name_offset() as usize)
        .ok_or(Error::InvalidStringOffset(
          attribute_info.name_index(),
          attribute_info.name_offset(),
        ))?;
      node_data
        .attributes_mut()
        .insert(attribute_name.to_owned(), attribute);

      if attribute_info.next_attribute_index() == -1 {
        break;
      }

      attribute_info = context
        .attribute_infos()
        .get(attribute_info.next_attribute_index() as usize)
        .ok_or(Error::InvalidAttributeIndex(
          attribute_info.next_attribute_index(),
        ))?;
    }
    Ok(node_data)
  }

  fn read_lsf_attribute(
    &mut self,
    attribute_info: &AttributeInfo,
    context: &Context,
  ) -> Result<Attribute, Error> {
    let data = self.read_lsf_attribute_value(
      attribute_info.length() as usize,
      attribute_info.type_id(),
      context.header().version(),
    )?;
    Ok(Attribute::new_value(data))
  }

  fn read_lsf_attribute_value(
    &mut self,
    size: usize,
    type_id: u32,
    lsf_version: &Version,
  ) -> Result<Value, Error> {
    match type_id {
      20 => Ok(Value::String(self.read_utf8_string(size)?)),
      21 => Ok(Value::Path(self.read_utf8_string(size)?)),
      22 => Ok(Value::FixedString(self.read_utf8_string(size)?)),
      23 => Ok(Value::LsString(self.read_utf8_string(size)?)),
      25 => Ok(Value::ScratchBuffer(self.read_bytes(size)?)),
      28 => {
        let (version, value) = if lsf_version >= &Version::V4 {
          (self.read_u16::<LittleEndian>()?, "".to_owned())
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
      29 => Ok(Value::WString(self.read_utf8_string(size)?)),
      30 => Ok(Value::LswString(self.read_utf8_string(size)?)),
      33 => Ok(self.read_lsf_translated_fs_string(lsf_version)?),
      _ => self.read_resource_value(type_id),
    }
  }

  fn read_lsf_translated_fs_string(&mut self, lsf_version: &Version) -> Result<Value, Error> {
    let (version, value) = if lsf_version >= &Version::V4 {
      (self.read_u16::<LittleEndian>()?, "".to_owned())
    } else {
      let value_length = self.read_i32::<LittleEndian>()?;
      let value = self.read_utf8_string(value_length as usize)?;
      (0u16, value)
    };
    let handle_length = self.read_i32::<LittleEndian>()?;
    let handle = self.read_utf8_string(handle_length as usize)?;
    let argument_count = self.read_i32::<LittleEndian>()?;
    let mut arguments: Vec<TranslatedFsStringArgument> =
      Vec::with_capacity(argument_count as usize);

    for _ in 0..argument_count {
      let arg_key_length = self.read_i32::<LittleEndian>()?;
      let arg_key = self.read_utf8_string(arg_key_length as usize)?;
      let arg_string = self.read_lsf_translated_fs_string(lsf_version)?;
      let arg_value_length = self.read_i32::<LittleEndian>()?;
      let arg_value = self.read_utf8_string(arg_value_length as usize)?;
      arguments.push(TranslatedFsStringArgument {
        key: arg_key,
        string: arg_string,
        value: arg_value,
      });
    }

    Ok(Value::TranslatedFsString {
      version,
      value,
      handle,
      arguments,
    })
  }
}

impl<R: Read + Seek + ?Sized> LsfReadExt for R {}
