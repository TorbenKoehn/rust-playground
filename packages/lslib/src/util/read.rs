use std::{io::Read, str::from_utf8};

use crate::error::Error;

pub trait BinaryReadExt: Read {
  fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![0; length];
    self.read_exact(&mut buffer)?;
    Ok(buffer)
  }

  fn read_utf8_string(&mut self, length: usize) -> Result<String, Error> {
    let buffer = self.read_bytes(length)?;
    Ok(from_utf8(&buffer)?.trim_end_matches('\0').to_string())
  }
}

impl<R: Read + ?Sized> BinaryReadExt for R {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_bytes() {
    let mut reader = std::io::Cursor::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(reader.read_bytes(0).unwrap(), vec![0u8; 0]);
    assert_eq!(reader.read_bytes(1).unwrap(), vec![0]);
    assert_eq!(reader.read_bytes(2).unwrap(), vec![1, 2]);
    assert_eq!(reader.read_bytes(3).unwrap(), vec![3, 4, 5]);
    assert_eq!(reader.read_bytes(4).unwrap(), vec![6, 7, 8, 9]);
    assert_eq!(reader.read_bytes(0).unwrap(), vec![0u8; 0]);
  }

  #[test]
  fn test_read_utf8_string() {
    let mut reader = std::io::Cursor::new(b"Hello, World!!!\0".to_vec());
    assert_eq!(reader.read_utf8_string(0).unwrap(), "");
    assert_eq!(reader.read_utf8_string(1).unwrap(), "H");
    assert_eq!(reader.read_utf8_string(2).unwrap(), "el");
    assert_eq!(reader.read_utf8_string(3).unwrap(), "lo,");
    assert_eq!(reader.read_utf8_string(4).unwrap(), " Wor");
    assert_eq!(reader.read_utf8_string(5).unwrap(), "ld!!!");
    assert_eq!(reader.read_utf8_string(0).unwrap(), "");
  }

  #[test]
  fn test_read_utf8_string_strips_and_ignores_null_terminators() {
    let mut reader = std::io::Cursor::new(b"Hello, World!!!\0\0\0".to_vec());
    assert_eq!(reader.read_utf8_string(18).unwrap(), "Hello, World!!!");
  }
}
