pub mod read;

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum Method {
  None = 0x00,
  Zlib = 0x01,
  Lz4 = 0x02,
}

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum Level {
  Fast = 0x10,
  Default = 0x20,
  Max = 0x30,
}

#[derive(Debug)]
pub struct CompressionOptions {
  method: Method,
  level: Level,
  chunked: bool,
  compressed_crc: Option<u32>,
}

impl CompressionOptions {
  pub fn new(method: Method, level: Level) -> Self {
    Self {
      method,
      level,
      chunked: false,
      compressed_crc: None,
    }
  }

  pub fn method(&self) -> &Method {
    &self.method
  }

  pub fn level(&self) -> &Level {
    &self.level
  }

  pub fn chunked(&self) -> bool {
    self.chunked
  }

  pub fn set_chunked(&mut self, chunked: bool) {
    self.chunked = chunked;
  }

  pub fn compressed_crc(&self) -> Option<u32> {
    self.compressed_crc
  }

  pub fn set_compressed_crc(&mut self, crc: Option<u32>) {
    self.compressed_crc = crc;
  }
}

impl From<u8> for CompressionOptions {
  fn from(value: u8) -> Self {
    let method = match value & 0x0f {
      0x01 => Method::Zlib,
      0x02 => Method::Lz4,
      _ => Method::None,
    };
    let level = match value & 0xf0 {
      0x10 => Level::Fast,
      0x20 => Level::Default,
      0x30 => Level::Max,
      _ => Level::Default,
    };
    Self {
      method,
      level,
      chunked: false,
      compressed_crc: None,
    }
  }
}

impl Into<u8> for CompressionOptions {
  fn into(self) -> u8 {
    let mut value = 0u8;
    value |= match self.method {
      Method::Zlib => 0x01,
      Method::Lz4 => 0x02,
      _ => 0,
    };
    value |= match self.level {
      Level::Fast => 0x10,
      Level::Default => 0x20,
      Level::Max => 0x30,
    };
    value
  }
}

impl From<Method> for CompressionOptions {
  fn from(method: Method) -> Self {
    Self {
      method,
      level: Level::Default,
      chunked: false,
      compressed_crc: None,
    }
  }
}
