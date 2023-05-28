use std::fmt::Debug;

use super::index::Index;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<V> {
  index: Index,
  parent_index: Option<Index>,
  child_indexes: Vec<Index>,
  value: V,
}

impl<V> Node<V> {
  pub fn new(index: Index, value: V, parent_index: Option<Index>) -> Self {
    Self {
      index,
      parent_index,
      child_indexes: Vec::new(),
      value,
    }
  }

  pub fn index(&self) -> Index {
    self.index
  }

  pub fn parent_index(&self) -> Option<Index> {
    self.parent_index
  }

  pub fn child_indexes(&self) -> &Vec<Index> {
    &self.child_indexes
  }

  pub fn child_indexes_mut(&mut self) -> &mut Vec<Index> {
    &mut self.child_indexes
  }

  pub fn value(&self) -> &V {
    &self.value
  }

  pub fn value_mut(&mut self) -> &mut V {
    &mut self.value
  }
}

impl<V> Debug for Node<V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Node")
      .field("index", &self.index)
      .field("parent_index", &self.parent_index)
      .field("child_indexes", &self.child_indexes)
      .finish()
  }
}
