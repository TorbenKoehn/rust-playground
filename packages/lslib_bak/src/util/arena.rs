pub type Index = usize;

#[derive(Clone, Debug)]
pub struct Node<V> {
  index: Index,
  parent_index: Option<Index>,
  child_indexes: Vec<Index>,
  value: V,
}

impl<V> Node<V> {
  pub fn index(&self) -> Index {
    self.index
  }

  pub fn parent_index(&self) -> Option<Index> {
    self.parent_index
  }

  pub fn child_indexes(&self) -> &Vec<Index> {
    &self.child_indexes
  }

  pub fn value(&self) -> &V {
    &self.value
  }

  pub fn value_mut(&mut self) -> &mut V {
    &mut self.value
  }
}

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

pub struct RecursiveIter<'a, V> {
  arena: &'a Arena<V>,
  stack: Vec<Index>,
}

impl<'a, V> RecursiveIter<'a, V> {
  pub fn new(arena: &'a Arena<V>, index: Index) -> Self {
    let mut stack = Vec::with_capacity(arena.nodes.len());
    stack.push(index);
    Self { arena, stack }
  }
}

impl<'a, V> Iterator for RecursiveIter<'a, V> {
  type Item = Index;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(index) = self.stack.pop() {
      let node = &self.arena.nodes[index];
      self.stack.extend(node.child_indexes.iter().rev());
      Some(node.index)
    } else {
      None
    }
  }
}

pub trait ArenaReader<V> {
  fn size(&self) -> usize;
  fn parent_index(&self, index: Index) -> Option<Index>;
  fn child_indexes(&self, index: Index) -> &Vec<Index>;
  fn value(&self, index: Index) -> &V;
  fn root_indexes(&self) -> &Vec<Index>;
  fn recursive_iter(&self, index: Index) -> RecursiveIter<'_, V>;
}

impl<V> ArenaReader<V> for Arena<V> {
  fn size(&self) -> usize {
    self.nodes.len()
  }

  fn parent_index(&self, index: Index) -> Option<Index> {
    self.nodes[index].parent_index
  }

  fn child_indexes(&self, index: Index) -> &Vec<Index> {
    &self.nodes[index].child_indexes
  }

  fn value(&self, index: Index) -> &V {
    &self.nodes[index].value
  }

  fn root_indexes(&self) -> &Vec<Index> {
    &self.root_indexes
  }

  fn recursive_iter(&self, index: Index) -> RecursiveIter<'_, V> {
    RecursiveIter::new(self, index)
  }
}

pub trait ArenaWriter<V> {
  fn alloc(&mut self, value: V, parent: Option<Index>) -> Index;
  fn value_mut(&mut self, index: Index) -> &mut V;
}

impl<V> ArenaWriter<V> for Arena<V> {
  fn alloc(&mut self, value: V, parent: Option<Index>) -> Index {
    let index = self.nodes.len();
    self.nodes.push(Node {
      index,
      value,
      parent_index: parent,
      child_indexes: Vec::new(),
    });
    if let Some(parent_index) = parent {
      self.nodes[parent_index].child_indexes.push(index);
    } else {
      self.root_indexes.push(index);
    }
    index
  }

  fn value_mut(&mut self, index: Index) -> &mut V {
    &mut self.nodes[index].value
  }
}

#[cfg(test)]
mod tests {
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
