use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct NodeInfo {
  parent_index: i32,
  name_index: i32,
  name_offset: i32,
  first_attribute_index: i32,
}

impl NodeInfo {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn parent_index(&self) -> i32 {
    self.parent_index
  }

  pub fn set_parent_index(&mut self, parent_index: i32) {
    self.parent_index = parent_index;
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

  pub fn first_attribute_index(&self) -> i32 {
    self.first_attribute_index
  }

  pub fn set_first_attribute_index(&mut self, first_attribute_index: i32) {
    self.first_attribute_index = first_attribute_index;
  }
}

impl Default for NodeInfo {
  fn default() -> Self {
    Self {
      parent_index: -1,
      name_index: -1,
      name_offset: -1,
      first_attribute_index: -1,
    }
  }
}
