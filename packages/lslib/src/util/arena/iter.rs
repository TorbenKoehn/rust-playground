use super::{index::Index, Arena};

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
      let node = &self.arena.nodes[index.into_usize()];
      self.stack.extend(node.child_indexes().iter().rev());
      Some(node.index())
    } else {
      None
    }
  }
}
