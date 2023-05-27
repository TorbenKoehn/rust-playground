pub enum Error {
  Runtime(String),
  Logic(String),
  Io(std::io::Error),
  Env(std::env::VarError),
}

impl Error {
  pub fn message(&self) -> String {
    match self {
      Self::Runtime(message) => message.to_string(),
      Self::Logic(message) => message.to_string(),
      Self::Io(error) => error.to_string(),
      Self::Env(error) => error.to_string(),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::Io(error)
  }
}

impl From<std::env::VarError> for Error {
  fn from(error: std::env::VarError) -> Self {
    Error::Env(error)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl std::fmt::Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl std::error::Error for Error {}
