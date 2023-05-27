use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
  ImageBuildFailure(String),
  ContainerCreateFailure(String),
  ContainerStartFailure(String),
  Io(std::io::Error),
  Yaml(serde_yaml::Error),
  Json(serde_json::Error),
  Bollard(bollard::errors::Error),
}

impl Error {
  pub fn message(&self) -> String {
    match self {
      Self::ImageBuildFailure(message) => message.to_string(),
      Self::ContainerCreateFailure(message) => message.to_string(),
      Self::ContainerStartFailure(message) => message.to_string(),
      Self::Io(error) => error.to_string(),
      Self::Yaml(error) => error.to_string(),
      Self::Json(error) => error.to_string(),
      Self::Bollard(error) => error.to_string(),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
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

impl From<bollard::errors::Error> for Error {
  fn from(error: bollard::errors::Error) -> Self {
    Error::Bollard(error)
  }
}
