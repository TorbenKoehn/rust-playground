use std::collections::HashMap;

use serde::Serialize;

use crate::resource::value::Value;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Attribute {
  #[serde(flatten)]
  value: Value,
}

impl Attribute {
  pub fn new() -> Self {
    Self { value: Value::None }
  }

  pub fn new_value(value: Value) -> Self {
    Self { value }
  }

  pub fn value(&self) -> &Value {
    &self.value
  }

  pub fn set_value(&mut self, value: Value) {
    self.value = value;
  }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AttributeMap {
  #[serde(flatten)]
  attributes: HashMap<String, Attribute>,
}

impl AttributeMap {
  pub fn new() -> Self {
    Self {
      attributes: HashMap::new(),
    }
  }

  pub fn get(&self, key: &str) -> Option<&Attribute> {
    self.attributes.get(key)
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut Attribute> {
    self.attributes.get_mut(key)
  }

  pub fn insert(&mut self, key: String, value: Attribute) {
    self.attributes.insert(key, value);
  }

  pub fn remove(&mut self, key: &str) -> Option<Attribute> {
    self.attributes.remove(key)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&String, &Attribute)> {
    self.attributes.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Attribute)> {
    self.attributes.iter_mut()
  }
}
