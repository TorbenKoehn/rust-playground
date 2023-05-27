use std::fmt::Debug;

pub type Signature = [u8; 4];

pub const SIGNATURE: Signature = [0x4c, 0x53, 0x4f, 0x46]; // "LSOF"

#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Version {
  V1 = 1,
  V2 = 2,
  V3 = 3,
  V4 = 4, // BG3
  V5 = 5,
  V6 = 6,
}

impl From<u32> for Version {
  fn from(version: u32) -> Self {
    match version {
      1 => Version::V1,
      2 => Version::V2,
      3 => Version::V3,
      4 => Version::V4,
      5 => Version::V5,
      6 => Version::V6,
      _ => panic!("Invalid package version: {}", version),
    }
  }
}

pub struct Header {
  version: Version,
  engine_version: i64,
  strings_uncompressed_size: u32,
  strings_size_on_disk: u32,
  nodes_uncompressed_size: u32,
  nodes_size_on_disk: u32,
  attributes_uncompressed_size: u32,
  attributes_size_on_disk: u32,
  values_uncompressed_size: u32,
  values_size_on_disk: u32,
  compression_flags: u8,
  has_sibling_data: u32,
}

impl Header {
  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn set_version(&mut self, version: Version) {
    self.version = version;
  }

  pub fn engine_version(&self) -> i64 {
    self.engine_version
  }

  pub fn set_engine_version(&mut self, engine_version: i64) {
    self.engine_version = engine_version;
  }

  pub fn strings_uncompressed_size(&self) -> u32 {
    self.strings_uncompressed_size
  }

  pub fn set_strings_uncompressed_size(&mut self, strings_uncompressed_size: u32) {
    self.strings_uncompressed_size = strings_uncompressed_size;
  }

  pub fn strings_size_on_disk(&self) -> u32 {
    self.strings_size_on_disk
  }

  pub fn set_strings_size_on_disk(&mut self, strings_size_on_disk: u32) {
    self.strings_size_on_disk = strings_size_on_disk;
  }

  pub fn nodes_uncompressed_size(&self) -> u32 {
    self.nodes_uncompressed_size
  }

  pub fn set_nodes_uncompressed_size(&mut self, nodes_uncompressed_size: u32) {
    self.nodes_uncompressed_size = nodes_uncompressed_size;
  }

  pub fn nodes_size_on_disk(&self) -> u32 {
    self.nodes_size_on_disk
  }

  pub fn set_nodes_size_on_disk(&mut self, nodes_size_on_disk: u32) {
    self.nodes_size_on_disk = nodes_size_on_disk;
  }

  pub fn attributes_uncompressed_size(&self) -> u32 {
    self.attributes_uncompressed_size
  }

  pub fn set_attributes_uncompressed_size(&mut self, attributes_uncompressed_size: u32) {
    self.attributes_uncompressed_size = attributes_uncompressed_size;
  }

  pub fn attributes_size_on_disk(&self) -> u32 {
    self.attributes_size_on_disk
  }

  pub fn set_attributes_size_on_disk(&mut self, attributes_size_on_disk: u32) {
    self.attributes_size_on_disk = attributes_size_on_disk;
  }

  pub fn values_uncompressed_size(&self) -> u32 {
    self.values_uncompressed_size
  }

  pub fn set_values_uncompressed_size(&mut self, values_uncompressed_size: u32) {
    self.values_uncompressed_size = values_uncompressed_size;
  }

  pub fn values_size_on_disk(&self) -> u32 {
    self.values_size_on_disk
  }

  pub fn set_values_size_on_disk(&mut self, values_size_on_disk: u32) {
    self.values_size_on_disk = values_size_on_disk;
  }

  pub fn compression_flags(&self) -> u8 {
    self.compression_flags
  }

  pub fn set_compression_flags(&mut self, compression_flags: u8) {
    self.compression_flags = compression_flags;
  }

  pub fn has_sibling_data(&self) -> u32 {
    self.has_sibling_data
  }

  pub fn set_has_sibling_data(&mut self, has_sibling_data: u32) {
    self.has_sibling_data = has_sibling_data;
  }
}

impl Default for Header {
  fn default() -> Self {
    Self {
      version: Version::V6,
      engine_version: 0,
      strings_uncompressed_size: 0,
      strings_size_on_disk: 0,
      nodes_uncompressed_size: 0,
      nodes_size_on_disk: 0,
      attributes_uncompressed_size: 0,
      attributes_size_on_disk: 0,
      values_uncompressed_size: 0,
      values_size_on_disk: 0,
      compression_flags: 0,
      has_sibling_data: 0,
    }
  }
}

impl Debug for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Header")
      .field("version", &self.version)
      .field("engine_version", &self.engine_version)
      .field("strings_uncompressed_size", &self.strings_uncompressed_size)
      .field("strings_size_on_disk", &self.strings_size_on_disk)
      .field("nodes_uncompressed_size", &self.nodes_uncompressed_size)
      .field("nodes_size_on_disk", &self.nodes_size_on_disk)
      .field(
        "attributes_uncompressed_size",
        &self.attributes_uncompressed_size,
      )
      .field("attributes_size_on_disk", &self.attributes_size_on_disk)
      .field("values_uncompressed_size", &self.values_uncompressed_size)
      .field("values_size_on_disk", &self.values_size_on_disk)
      .field("compression_flags", &self.compression_flags)
      .field("has_sibling_data", &self.has_sibling_data)
      .finish()
  }
}
