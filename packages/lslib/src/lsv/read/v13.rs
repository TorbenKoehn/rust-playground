use std::{
  io::{Cursor, Read, Seek, SeekFrom},
  path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use lz4_flex::decompress;

use crate::{
  compression::{read::DecompressReadExt, CompressionOptions, Method},
  error::Error,
  lsv::{file::File, header::Header, package::Package},
  util::read::BinaryReadExt,
};

pub fn read_lsv_v13_file_contents<R: Read + Seek>(
  readers: &mut Vec<R>,
  header: &Header,
  file: &File,
) -> Result<Vec<u8>, Error> {
  let contents = {
    if file.size_on_disk() > 0x7fffffff {
      return Err(Error::FileTooLarge(
        file.path().to_str().unwrap().to_owned(),
        file.size_on_disk(),
      ));
    }

    if header.is_solid() && file.contents().is_none() {
      return Err(Error::FileEmpty(file.path().to_str().unwrap().to_owned()));
    }

    let reader = readers
      .get_mut(file.part_index() as usize)
      .ok_or(Error::InvalidFileTable)?;

    if file.is_compressed() {
      reader.seek(SeekFrom::Start(file.offset() as u64))?;

      let mut compression_options: CompressionOptions = file.compression_method().into();
      compression_options.set_compressed_crc(Some(file.crc()));
      let uncompressed_bytes = reader.read_decompressed(
        file.size_on_disk() as usize,
        file.uncompressed_size() as usize,
        compression_options,
      )?;
      Ok::<Vec<u8>, Error>(uncompressed_bytes)
    } else {
      reader.seek(SeekFrom::Start(file.offset() as u64))?;
      let uncompressed_bytes = reader.read_bytes(file.size_on_disk() as usize)?;
      Ok(uncompressed_bytes)
    }?
  };

  Ok(contents)
}

pub trait LsvV13ReadExt: Read + Seek {
  fn read_lsv_v13_header(&mut self, header: &mut Header) -> Result<(), Error> {
    self.seek(SeekFrom::End(-8))?;
    let header_size = self.read_i32::<LittleEndian>()? as i64;
    self.seek(SeekFrom::End(-header_size))?;

    let version = self.read_u32::<LittleEndian>()?;
    let file_table_offset = self.read_u32::<LittleEndian>()?;
    let file_table_size = self.read_u32::<LittleEndian>()?;
    let part_count = self.read_u16::<LittleEndian>()?;
    let flags = self.read_u8()?;
    let priority = self.read_u8()?;
    let _md5_hash = {
      let mut hash = [0; 16];
      self.read_exact(&mut hash)?;
      hash
    };

    header.set_version(version.into());
    header.set_part_count(part_count);
    header.set_file_table_offset(file_table_offset);
    header.set_file_table_size(file_table_size);
    header.set_flags(flags);
    header.set_priority(priority);

    Ok(())
  }

  fn read_lsv_v13_file_table(&mut self, package: &mut Package) -> Result<(), Error> {
    let is_solid = package.header().is_solid();
    self.seek(SeekFrom::Start(package.header().file_table_offset() as u64))?;
    let file_count = self.read_i32::<LittleEndian>()?;
    let compressed_bytes = self.read_bytes(package.header().file_table_size() as usize - 4)?;
    let uncompressed_size = (256 + 6 * 4) * file_count as usize;
    let uncompressed_bytes = decompress(&compressed_bytes, uncompressed_size)?;

    let mut cursor = Cursor::new(&uncompressed_bytes);

    package.files_mut().reserve(file_count as usize);

    for _ in 0..file_count {
      let path = cursor.read_utf8_string(256)?;
      let offset = cursor.read_u32::<LittleEndian>()?;
      let size_on_disk = cursor.read_u32::<LittleEndian>()?;
      let uncompressed_size = cursor.read_u32::<LittleEndian>()?;
      let part_index = cursor.read_u32::<LittleEndian>()?;
      let flags = cursor.read_u32::<LittleEndian>()?;
      let crc = cursor.read_u32::<LittleEndian>()?;

      let mut file = File::new();
      file.set_path(PathBuf::from(path));
      file.set_offset(offset);
      file.set_size_on_disk(size_on_disk);
      file.set_uncompressed_size(uncompressed_size);
      file.set_part_index(part_index);
      file.set_flags(flags);
      file.set_crc(crc);
      package.files_mut().insert(file.path().to_owned(), file);
    }

    // If this is a solid archive, we directly decompress the entire file table
    if is_solid {
      let mut total_size_on_disk = 0u32;
      let mut first_offset = 0xffffffffu32;
      let mut last_offset = 0u32;
      for (_, file) in package.files() {
        if file.offset() < first_offset {
          first_offset = file.offset();
        }
        if file.offset() > last_offset {
          last_offset = file.offset();
        }
        total_size_on_disk += file.size_on_disk();
      }

      if first_offset != 7 || last_offset - first_offset != total_size_on_disk {
        return Err(Error::InvalidFileTable);
      }

      // Decompress as a single frame (solid)
      self.seek(SeekFrom::Start(0))?;
      let mut compression_options: CompressionOptions = Method::Lz4.into();
      compression_options.set_chunked(true);
      let uncompressed_frame = self.read_decompressed(
        last_offset as usize,
        total_size_on_disk as usize,
        compression_options,
      )?;

      let mut solid_cursor = Cursor::new(uncompressed_frame);
      solid_cursor.seek(SeekFrom::Start(first_offset as u64))?;

      let file_names: Vec<PathBuf> = package.files().keys().cloned().collect();
      for file_path in file_names {
        let file = package.file_mut(&file_path).unwrap();
        if file.offset() != solid_cursor.position() as u32 {
          return Err(Error::InvalidFileTable);
        }

        let contents = solid_cursor.read_bytes(file.uncompressed_size() as usize)?;
        file.set_contents(Some(contents));
      }
    }

    Ok(())
  }
}

impl<R: Read + Seek + ?Sized> LsvV13ReadExt for R {}
