use std::io::Write;

use crate::error::Error;

pub trait BinaryWriteExt: Write {
  fn write_bytes(&mut self, bytes: &Vec<u8>) -> Result<(), Error> {
    self.write_all(bytes)?;
    Ok(())
  }

  fn write_utf8_string(&mut self, value: &str, length: usize) -> Result<(), Error> {
    // Fill string with null bytes until it has "length" size
    let mut buffer = vec![0u8; length];
    buffer[..value.len()].copy_from_slice(value.as_bytes());
    self.write_all(&buffer)?;
    Ok(())
  }
}

impl<R: Write + ?Sized> BinaryWriteExt for R {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_write_bytes() {
    let mut writer = std::io::Cursor::new(vec![]);
    writer.write_bytes(&vec![0x00, 0x01, 0x02, 0x03]).unwrap();
    assert_eq!(writer.into_inner(), vec![0x00, 0x01, 0x02, 0x03]);
  }

  #[test]
  fn test_write_utf8_string() {
    let mut writer = std::io::Cursor::new(vec![]);
    writer.write_utf8_string("Hello, World!!!", 18).unwrap();
    assert_eq!(writer.into_inner(), b"Hello, World!!!\0\0\0".to_vec());
  }
}
