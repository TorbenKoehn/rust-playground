use std::io::{Cursor, Read};

use crc32fast::hash;
use flate2::read::ZlibDecoder;
use lz4_flex::{block, frame};

use crate::{error::Error, util::read::BinaryReadExt};

use super::{CompressionOptions, Method};

pub trait DecompressReadExt: Read {
  fn read_decompressed(
    &mut self,
    compressed_size: usize,
    uncompressed_size: usize,
    options: CompressionOptions,
  ) -> Result<Vec<u8>, Error> {
    let bytes = self.read_bytes(compressed_size)?;
    if let Some(expected_crc) = options.compressed_crc() {
      let crc = hash(&bytes);
      if crc != expected_crc {
        return Err(Error::CrcMismatch(expected_crc, crc));
      }
    }
    match options {
      CompressionOptions {
        method: Method::None,
        ..
      } => Ok(bytes),
      CompressionOptions {
        method: Method::Zlib,
        ..
      } => Ok(ZlibDecoder::new(Cursor::new(bytes)).read_bytes(uncompressed_size)?),
      CompressionOptions {
        method: Method::Lz4,
        chunked: true,
        ..
      } => Ok(frame::FrameDecoder::new(Cursor::new(bytes)).read_bytes(uncompressed_size)?),
      CompressionOptions {
        method: Method::Lz4,
        ..
      } => Ok(block::decompress(&bytes, uncompressed_size)?),
    }
  }
}

impl<R: Read + ?Sized> DecompressReadExt for R {}
