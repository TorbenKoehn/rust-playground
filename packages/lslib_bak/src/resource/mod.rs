pub mod metadata;
pub mod node;
pub mod reader;
pub mod value;
pub mod writer;

use std::fmt::Debug;

use serde::{ser::SerializeStruct, Serialize};

use crate::util::arena::{Arena, ArenaReader};

use self::node::data::Data;

pub struct Resource {
  arena: Arena<Data>,
}

impl Resource {
  pub fn new() -> Self {
    Self {
      arena: Arena::new(),
    }
  }

  pub fn new_with_arena(arena: Arena<Data>) -> Self {
    Self { arena }
  }
}

impl Debug for Resource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Resource")
      .field("node_count", &self.size())
      .field("roots", &self.arena.root_indexes())
      .finish()
  }
}

impl Serialize for Resource {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let mut resource = serializer.serialize_struct("Resource", 2)?;
    resource.serialize_field("roots", &self.root_indexes())?;
    resource.end()
  }
}

#[cfg(test)]
mod tests {
  use crate::util::arena::ArenaWriter;

  use super::*;

  #[test]
  fn test_resource() {
    let mut arena = Resource::new();
    let root = arena.alloc(Data::new("Root".to_owned()), None);
    let child = arena.alloc(Data::new("ChildA".to_owned()), Some(root));
    assert_eq!(arena.size(), 2);
    assert_eq!(arena.parent_index(child), Some(root));
    assert_eq!(arena.child_indexes(root), &vec![child]);
    assert_eq!(arena.value(root), &Data::new("Root".to_owned()));
    assert_eq!(arena.value(child), &Data::new("ChildA".to_owned()));
    assert_eq!(arena.root_indexes(), &vec![root]);
    assert_eq!(
      arena.recursive_iter(root).collect::<Vec<_>>(),
      vec![root, child]
    );
  }
}
