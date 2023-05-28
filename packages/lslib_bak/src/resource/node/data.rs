use super::attribute::AttributeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  Element,
  Region(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
  kind: Kind,
  name: String,
  attributes: AttributeMap,
}

impl Data {
  pub fn new(name: String) -> Self {
    Self {
      kind: Kind::Element,
      name,
      attributes: AttributeMap::new(),
    }
  }

  pub fn new_with_attributes(name: String, attributes: AttributeMap) -> Self {
    Self {
      kind: Kind::Element,
      name,
      attributes,
    }
  }

  pub fn new_region(name: String, region: String) -> Self {
    Self {
      kind: Kind::Region(region),
      name,
      attributes: AttributeMap::new(),
    }
  }

  pub fn kind(&self) -> &Kind {
    &self.kind
  }

  pub fn set_kind(&mut self, kind: Kind) {
    self.kind = kind;
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn attributes(&self) -> &AttributeMap {
    &self.attributes
  }

  pub fn attributes_mut(&mut self) -> &mut AttributeMap {
    &mut self.attributes
  }
}
