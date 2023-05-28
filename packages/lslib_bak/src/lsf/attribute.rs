use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct AttributeInfo {
  name_index: i32,
  name_offset: i32,
  type_id: u32,
  length: u32,
  data_offset: u32,
  next_attribute_index: i32,
}

impl AttributeInfo {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn name_index(&self) -> i32 {
    self.name_index
  }

  pub fn set_name_index(&mut self, name_index: i32) {
    self.name_index = name_index;
  }

  pub fn name_offset(&self) -> i32 {
    self.name_offset
  }

  pub fn set_name_offset(&mut self, name_offset: i32) {
    self.name_offset = name_offset;
  }

  pub fn type_id(&self) -> u32 {
    self.type_id
  }

  pub fn set_type_id(&mut self, type_id: u32) {
    self.type_id = type_id;
  }

  pub fn length(&self) -> u32 {
    self.length
  }

  pub fn set_length(&mut self, length: u32) {
    self.length = length;
  }

  pub fn data_offset(&self) -> u32 {
    self.data_offset
  }

  pub fn set_data_offset(&mut self, data_offset: u32) {
    self.data_offset = data_offset;
  }

  pub fn next_attribute_index(&self) -> i32 {
    self.next_attribute_index
  }

  pub fn set_next_attribute_index(&mut self, next_attribute_index: i32) {
    self.next_attribute_index = next_attribute_index;
  }
}

impl Default for AttributeInfo {
  fn default() -> Self {
    Self {
      name_index: -1,
      name_offset: -1,
      type_id: 0,
      length: 0,
      data_offset: 0,
      next_attribute_index: -1,
    }
  }
}
