use std::{fmt::Debug, io::ErrorKind, str::from_utf8};

pub enum Error {
  InvalidSignature([u8; 4], [u8; 4]),
  InvalidVersion(i32),
  InvalidFileTable,
  FileTooLarge(String, u32),
  CrcMismatch(u32, u32),
  FileNotFound(String),
  FileEmpty(String),
  InvalidStringIndex(i32),
  InvalidStringOffset(i32, i32),
  InvalidAttributeIndex(i32),
  InvalidTypeId(u32),
  InvalidPath(String),
  Io(std::io::Error),
  Lz4Decompress(lz4_flex::block::DecompressError),
  Utf8(std::str::Utf8Error),
  Yaml(serde_yaml::Error),
}

impl Error {
  pub fn message(&self) -> String {
    match self {
      Self::InvalidSignature(expected, actual) => format!(
        "Invalid signature: {} (expected: {}, actual: {})",
        from_utf8(actual).unwrap(),
        from_utf8(expected).unwrap(),
        from_utf8(actual).unwrap()
      ),
      Self::InvalidVersion(version) => format!("Invalid version: {}", version),
      Self::InvalidFileTable => "Invalid file table".to_string(),
      Self::FileTooLarge(path, size) => {
        format!("File too large: {} ({} bytes)", path, size)
      }
      Self::CrcMismatch(expected, actual) => format!(
        "CRC mismatch: {} (expected: {}, actual: {})",
        expected, expected, actual
      ),
      Self::FileNotFound(path) => format!("File not found: {}", path),
      Self::FileEmpty(path) => format!("File empty: {}", path),
      Self::InvalidStringIndex(index) => format!("Invalid string index: {}", index),
      Self::InvalidStringOffset(index, offset) => format!(
        "Invalid string offset: {} (index: {}, offset: {})",
        index, offset, offset
      ),
      Self::InvalidAttributeIndex(index) => {
        format!("Invalid attribute index: {}", index)
      }
      Self::InvalidTypeId(id) => format!("Invalid type ID: {}", id),
      Self::InvalidPath(path) => format!("Invalid path: {}", path),
      Self::Io(error) => match error.kind() {
        ErrorKind::NotFound => format!("File not found: {}", error),
        _ => format!("IO error: {} - {}", error.kind(), error),
      },
      Self::Lz4Decompress(error) => error.to_string(),
      Self::Utf8(error) => error.to_string(),
      Self::Yaml(error) => error.to_string(),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::Io(error)
  }
}

impl From<lz4_flex::block::DecompressError> for Error {
  fn from(error: lz4_flex::block::DecompressError) -> Self {
    Error::Lz4Decompress(error)
  }
}

impl From<std::str::Utf8Error> for Error {
  fn from(error: std::str::Utf8Error) -> Self {
    Error::Utf8(error)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Error::Yaml(error)
  }
}
