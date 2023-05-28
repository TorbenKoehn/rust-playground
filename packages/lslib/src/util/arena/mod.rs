use self::{index::Index, node::Node};

pub mod index;
pub mod iter;
pub mod node;
pub mod reader;
pub mod writer;

pub struct Arena<V> {
  nodes: Vec<Node<V>>,
  root_indexes: Vec<Index>,
}

impl<T> Arena<T> {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      root_indexes: Vec::new(),
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      nodes: Vec::with_capacity(capacity),
      root_indexes: Vec::new(),
    }
  }
}

impl<T> Arena<T> {
  pub fn size(&self) -> usize {
    self.nodes.len()
  }

  pub fn nodes(&self) -> &Vec<Node<T>> {
    &self.nodes
  }

  pub fn nodes_mut(&mut self) -> &mut Vec<Node<T>> {
    &mut self.nodes
  }

  pub fn root_indexes(&self) -> &Vec<Index> {
    &self.root_indexes
  }

  pub fn root_indexes_mut(&mut self) -> &mut Vec<Index> {
    &mut self.root_indexes
  }
}

#[cfg(test)]
mod tests {
  use crate::util::arena::{reader::ArenaReader, writer::ArenaWriter};

  use super::*;

  #[test]
  fn test_arena() {
    let mut arena = Arena::new();
    let root = arena.alloc(0, None);
    let child = arena.alloc(1, Some(root));
    assert_eq!(arena.size(), 2);
    assert_eq!(arena.parent_index(child), Some(root));
    assert_eq!(arena.child_indexes(root), &vec![child]);
    assert_eq!(arena.value(root), &0);
    assert_eq!(arena.value(child), &1);
    assert_eq!(arena.root_indexes(), &vec![root]);
    assert_eq!(
      arena.recursive_iter(root).collect::<Vec<_>>(),
      vec![root, child]
    );
  }
}
