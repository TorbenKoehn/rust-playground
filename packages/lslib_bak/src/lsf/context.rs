use std::fmt::Debug;

use super::{attribute::AttributeInfo, header::Header, node::NodeInfo};

pub struct Context {
  header: Header,
  string_lists: Vec<Vec<String>>,
  node_infos: Vec<NodeInfo>,
  attribute_infos: Vec<AttributeInfo>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      header: Default::default(),
      string_lists: Default::default(),
      node_infos: Default::default(),
      attribute_infos: Default::default(),
    }
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn header_mut(&mut self) -> &mut Header {
    &mut self.header
  }

  pub fn string_lists(&self) -> &Vec<Vec<String>> {
    &self.string_lists
  }

  pub fn string_lists_mut(&mut self) -> &mut Vec<Vec<String>> {
    &mut self.string_lists
  }

  pub fn node_infos(&self) -> &Vec<NodeInfo> {
    &self.node_infos
  }

  pub fn nodes_infos_mut(&mut self) -> &mut Vec<NodeInfo> {
    &mut self.node_infos
  }

  pub fn attribute_infos(&self) -> &Vec<AttributeInfo> {
    &self.attribute_infos
  }

  pub fn attribute_infos_mut(&mut self) -> &mut Vec<AttributeInfo> {
    &mut self.attribute_infos
  }
}

impl Debug for Context {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context")
      .field("header", &self.header)
      .field(
        "strings",
        &self
          .string_lists
          .iter()
          .fold(0, |sum, strings| sum + strings.len()),
      )
      .field("node_infos", &self.node_infos.len())
      .field("attribute_infos", &self.attribute_infos.len())
      .finish()
  }
}
