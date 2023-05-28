pub type Signature = [u8; 4];

pub const SIGNATURE: Signature = [0x4c, 0x53, 0x50, 0x4b]; // "LSPK"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Version {
  V7 = 7,   // D:OS 1
  V9 = 9,   // D:OS 1 EE
  V10 = 10, // D:OS 2
  V13 = 13, // D:OS 2 DE
  V15 = 15, // BG3 EA
  V16 = 16, // BG3 EA Patch4
}

impl From<i32> for Version {
  fn from(version: i32) -> Self {
    match version {
      7 => Version::V7,
      9 => Version::V9,
      10 => Version::V10,
      13 => Version::V13,
      15 => Version::V15,
      16 => Version::V16,
      _ => panic!("Invalid package version: {}", version),
    }
  }
}

impl From<u32> for Version {
  fn from(version: u32) -> Self {
    Self::from(version as i32)
  }
}

impl Default for Version {
  fn default() -> Self {
    Version::V16
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
  version: Version,
  // The number of parts this package consists of.
  part_count: u16,
  // Offset of the file table in the main archive
  file_table_offset: u32,
  file_table_size: u32,
  // Package flags bitmask. Allowed values are in the PackageFlags enumeration.
  flags: u8,
  // Load priority. Packages with higher priority are loaded later (i.e. they override earlier packages).
  priority: u8,
}

impl Header {
  pub const FLAGS_NONE: u8 = 0x00;
  // Allow memory-mapped access to the files in this archive.
  pub const FLAGS_ALLOW_MEMORY_MAPPING: u8 = 0x02;
  // All files are compressed into a single LZ4 stream
  pub const FLAGS_SOLID: u8 = 0x04;
  // Archive contents should be preloaded on game startup.
  pub const FLAGS_PRELOAD: u8 = 0x08;

  pub fn new() -> Self {
    Default::default()
  }

  pub fn version(&self) -> Version {
    self.version
  }

  pub fn set_version(&mut self, version: Version) {
    self.version = version;
  }

  pub fn part_count(&self) -> u16 {
    self.part_count
  }

  pub fn set_part_count(&mut self, part_count: u16) {
    self.part_count = part_count;
  }

  pub fn file_table_offset(&self) -> u32 {
    self.file_table_offset
  }

  pub fn set_file_table_offset(&mut self, file_table_offset: u32) {
    self.file_table_offset = file_table_offset;
  }

  pub fn file_table_size(&self) -> u32 {
    self.file_table_size
  }

  pub fn set_file_table_size(&mut self, file_table_size: u32) {
    self.file_table_size = file_table_size;
  }

  pub fn flags(&self) -> u8 {
    self.flags
  }

  pub fn set_flags(&mut self, flags: u8) {
    self.flags = flags;
  }

  pub fn priority(&self) -> u8 {
    self.priority
  }

  pub fn set_priority(&mut self, priority: u8) {
    self.priority = priority;
  }

  pub fn is_solid(&self) -> bool {
    self.flags & Header::FLAGS_SOLID != 0
  }
}

impl Default for Header {
  fn default() -> Self {
    Self {
      version: Version::default(),
      part_count: 0,
      file_table_offset: 0,
      file_table_size: 0,
      flags: Header::FLAGS_NONE,
      priority: 0,
    }
  }
}
