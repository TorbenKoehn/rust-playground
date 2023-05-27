use std::{
  collections::HashMap,
  fmt::{self, Debug, Formatter},
  path::{Path, PathBuf},
};

use crate::compression::Method;

#[derive(Default)]
pub struct File {
  path: PathBuf,
  offset: u32,
  size_on_disk: u32,
  uncompressed_size: u32,
  part_index: u32,
  flags: u32,
  crc: u32,
  contents: Option<Vec<u8>>,
}

impl File {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn set_path(&mut self, name: PathBuf) {
    self.path = name;
  }

  pub fn offset(&self) -> u32 {
    self.offset
  }

  pub fn set_offset(&mut self, offset: u32) {
    self.offset = offset;
  }

  pub fn size_on_disk(&self) -> u32 {
    self.size_on_disk
  }

  pub fn set_size_on_disk(&mut self, size_on_disk: u32) {
    self.size_on_disk = size_on_disk;
  }

  pub fn uncompressed_size(&self) -> u32 {
    self.uncompressed_size
  }

  pub fn set_uncompressed_size(&mut self, uncompressed_size: u32) {
    self.uncompressed_size = uncompressed_size;
  }

  pub fn part_index(&self) -> u32 {
    self.part_index
  }

  pub fn set_part_index(&mut self, archive_part: u32) {
    self.part_index = archive_part;
  }

  pub fn flags(&self) -> u32 {
    self.flags
  }

  pub fn set_flags(&mut self, flags: u32) {
    self.flags = flags;
  }

  pub fn crc(&self) -> u32 {
    self.crc
  }

  pub fn set_crc(&mut self, crc: u32) {
    self.crc = crc;
  }

  pub fn contents(&self) -> &Option<Vec<u8>> {
    &self.contents
  }

  pub fn set_contents(&mut self, contents: Option<Vec<u8>>) {
    self.contents = contents;
  }

  pub fn clear_contents(&mut self) {
    self.contents = None;
  }

  pub fn has_contents(&self) -> bool {
    self.contents.is_some()
  }

  pub fn compression_method(&self) -> Method {
    match self.flags & 0x0f {
      1 => Method::Zlib,
      2 => Method::Lz4,
      _ => Method::None,
    }
  }

  pub fn is_compressed(&self) -> bool {
    self.compression_method() != Method::None
  }
}

impl Debug for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("File")
      .field("path", &self.path)
      .field("size_on_disk", &self.size_on_disk)
      .field("uncompressed_size", &self.uncompressed_size)
      .field("part_index", &self.part_index)
      .finish()
  }
}

#[derive(Default)]
pub struct FileTable {
  files: HashMap<PathBuf, File>,
}

impl FileTable {
  pub fn new(size: usize) -> Self {
    Self {
      files: HashMap::with_capacity(size),
    }
  }

  pub fn files(&self) -> &HashMap<PathBuf, File> {
    &self.files
  }

  pub fn files_mut(&mut self) -> &mut HashMap<PathBuf, File> {
    &mut self.files
  }

  pub fn file(&self, path: &Path) -> Option<&File> {
    self.files.get(path)
  }

  pub fn file_mut(&mut self, path: &Path) -> Option<&mut File> {
    self.files.get_mut(path)
  }

  pub fn insert(&mut self, path: PathBuf, file: File) {
    self.files.insert(path, file);
  }

  pub fn remove(&mut self, path: &Path) -> Option<File> {
    self.files.remove(path)
  }
}
