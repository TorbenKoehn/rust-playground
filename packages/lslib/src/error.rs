use std::{fmt::Debug, io::ErrorKind};

pub enum Error {
  Io(std::io::Error),
  Lz4Decompress(lz4_flex::block::DecompressError),
  Utf8(std::str::Utf8Error),
  Yaml(serde_yaml::Error),
}

impl Error {
  pub fn message(&self) -> String {
    match self {
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
