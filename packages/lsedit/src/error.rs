use std::{fmt::Debug, io::ErrorKind};

use quick_xml::DeError;

pub enum Error {
  NoDataPath,
  Io(std::io::Error),
  Yaml(serde_yaml::Error),
  Json(serde_json::Error),
  Lslib(lslib::error::Error),
  XmlDeserialize(DeError),
}

impl Error {
  pub fn message(&self) -> String {
    match self {
      Self::NoDataPath => "No data path specified".to_string(),
      Self::Io(error) => match error.kind() {
        ErrorKind::NotFound => format!("File not found: {}", error),
        _ => format!("IO error: {} - {}", error.kind(), error),
      },
      Self::Yaml(error) => error.to_string(),
      Self::Json(error) => error.to_string(),
      Self::Lslib(error) => error.to_string(),
      Self::XmlDeserialize(error) => error.to_string(),
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

impl From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Error::Yaml(error)
  }
}

impl From<serde_json::Error> for Error {
  fn from(error: serde_json::Error) -> Self {
    Error::Json(error)
  }
}

impl From<lslib::error::Error> for Error {
  fn from(error: lslib::error::Error) -> Self {
    Error::Lslib(error)
  }
}

impl From<DeError> for Error {
  fn from(error: DeError) -> Self {
    Error::XmlDeserialize(error)
  }
}
