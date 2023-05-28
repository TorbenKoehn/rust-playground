use super::{index::Index, node::Node, Arena};

pub trait ArenaWriter<V> {
  fn alloc(&mut self, value: V, parent: Option<Index>) -> Index;
  fn value_mut(&mut self, index: Index) -> &mut V;
}

impl<V> ArenaWriter<V> for Arena<V> {
  fn alloc(&mut self, value: V, parent: Option<Index>) -> Index {
    // Find out next index for a node
    let index = Index::new(self.nodes.len());

    // Create and register node
    self.nodes.push(Node::new(index, value, parent));

    // Do order and classifications
    // Add to respective parents as child
    if let Some(parent_index) = parent {
      self.nodes[parent_index.into_usize()]
        .child_indexes_mut()
        .push(index);
    }

    // Classify as root nodes
    if let None = parent {
      self.root_indexes.push(index);
    }

    index
  }

  fn value_mut(&mut self, index: Index) -> &mut V {
    self.nodes[index.into_usize()].value_mut()
  }
}
