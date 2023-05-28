pub mod read;

#[derive(Default, Debug, Clone, Copy)]
pub struct Metadata {
  timestamp: u64,
  major_version: u32,
  minor_version: u32,
  revision: u32,
  build_number: u32,
}

impl Metadata {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }

  pub fn set_timestamp(&mut self, timestamp: u64) {
    self.timestamp = timestamp;
  }

  pub fn major_version(&self) -> u32 {
    self.major_version
  }

  pub fn set_major_version(&mut self, major_version: u32) {
    self.major_version = major_version;
  }

  pub fn minor_version(&self) -> u32 {
    self.minor_version
  }

  pub fn set_minor_version(&mut self, minor_version: u32) {
    self.minor_version = minor_version;
  }

  pub fn revision(&self) -> u32 {
    self.revision
  }

  pub fn set_revision(&mut self, revision: u32) {
    self.revision = revision;
  }

  pub fn build_number(&self) -> u32 {
    self.build_number
  }

  pub fn set_build_number(&mut self, build_number: u32) {
    self.build_number = build_number;
  }
}
