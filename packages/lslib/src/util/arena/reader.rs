use super::{index::Index, iter::RecursiveIter, Arena};

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
    self.nodes[index.into_usize()].parent_index()
  }

  fn child_indexes(&self, index: Index) -> &Vec<Index> {
    &self.nodes[index.into_usize()].child_indexes()
  }

  fn value(&self, index: Index) -> &V {
    &self.nodes[index.into_usize()].value()
  }

  fn root_indexes(&self) -> &Vec<Index> {
    &self.root_indexes
  }

  fn recursive_iter(&self, index: Index) -> RecursiveIter<'_, V> {
    RecursiveIter::new(self, index)
  }
}
