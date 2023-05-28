mod v13;

use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
  error::Error,
  lsv::header::{Header, Version},
};

use self::v13::{read_lsv_v13_file_contents, LsvV13ReadExt};

use super::{file::File, header::SIGNATURE, package::Package};

pub fn read_lsv_file_contents<R: Read + Seek>(
  readers: &mut Vec<R>,
  header: &Header,
  file: &File,
) -> Result<Vec<u8>, Error> {
  match header.version() {
    Version::V13 => read_lsv_v13_file_contents(readers, header, file),
    _ => todo!(
      "implement package reader for version {:?}",
      header.version()
    ),
  }
}

pub trait LsvReadExt: Read + Seek {
  fn read_lsv_package(&mut self, package: &mut Package) -> Result<(), Error> {
    self.read_lsv_header(package.header_mut())?;
    self.read_lsv_file_table(package)?;
    Ok(())
  }

  fn read_lsv_header(&mut self, header: &mut Header) -> Result<(), Error> {
    // Check for v13
    self.seek(SeekFrom::End(-4))?;
    let signature: [u8; 4] = self.read_u32::<LittleEndian>()?.to_le_bytes();
    if SIGNATURE == signature {
      self.read_lsv_v13_header(header)?;
      return Ok(());
    }

    // Check for v10, v15, v16
    self.seek(SeekFrom::Start(0))?;
    let signature: [u8; 4] = self.read_u32::<LittleEndian>()?.to_le_bytes();
    if SIGNATURE == signature {
      let version = self.read_i32::<LittleEndian>()?;
      match version {
        10 => todo!("implement v10 package reader"),
        15 => todo!("implement v15 package reader"),
        16 => todo!("implement v16 package reader"),
        _ => return Err(Error::InvalidVersion(version)),
      }
    }

    // Check for v7, v9
    self.seek(SeekFrom::Start(0))?;
    let version = self.read_i32::<LittleEndian>()?;
    match version {
      7 | 9 => todo!("implement v16 package reader"),
      _ => return Err(Error::InvalidVersion(version)),
    }
  }

  fn read_lsv_file_table(&mut self, package: &mut Package) -> Result<(), Error> {
    match package.header().version() {
      Version::V13 => self.read_lsv_v13_file_table(package),
      _ => todo!(
        "implement package reader for version {:?}",
        package.header().version()
      ),
    }?;
    Ok(())
  }
}

impl<R: Read + Seek + ?Sized> LsvReadExt for R {}
