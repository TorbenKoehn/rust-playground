use crate::util::arena::{ArenaWriter, Index};

use super::{node::data::Data, Resource};

impl ArenaWriter<Data> for Resource {
  fn alloc(&mut self, value: Data, parent: Option<Index>) -> Index {
    self.arena.alloc(value, parent)
  }

  fn value_mut(&mut self, index: Index) -> &mut Data {
    self.arena.value_mut(index)
  }
}

pub trait ResourceWriter: ArenaWriter<Data> {}

impl ResourceWriter for Resource {}
