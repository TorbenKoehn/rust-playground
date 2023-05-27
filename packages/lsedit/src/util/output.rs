use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
  Structure,
  Yaml,
  Json,
  Xml,
}

impl Display for OutputFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Structure => write!(f, "structure"),
      Self::Yaml => write!(f, "yaml"),
      Self::Json => write!(f, "json"),
      Self::Xml => write!(f, "xml"),
    }
  }
}
