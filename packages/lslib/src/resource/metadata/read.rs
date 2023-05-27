use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;

use super::Metadata;

pub trait ResourceMetadataReadExt: Read {
  fn read_resource_metadata(&mut self, metadata: &mut Metadata) -> Result<(), Error> {
    metadata.set_timestamp(self.read_u64::<LittleEndian>()?);
    metadata.set_major_version(self.read_u32::<LittleEndian>()?);
    metadata.set_minor_version(self.read_u32::<LittleEndian>()?);
    metadata.set_revision(self.read_u32::<LittleEndian>()?);
    metadata.set_build_number(self.read_u32::<LittleEndian>()?);
    Ok(())
  }
}

impl<R: Read + ?Sized> ResourceMetadataReadExt for R {}
