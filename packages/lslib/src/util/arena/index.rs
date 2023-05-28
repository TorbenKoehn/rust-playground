#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Index(usize);

impl Index {
  pub fn new(index: usize) -> Self {
    Self(index)
  }

  pub fn into_usize(self) -> usize {
    self.0
  }
}

impl Into<usize> for Index {
  fn into(self) -> usize {
    self.0
  }
}
