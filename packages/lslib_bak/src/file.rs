use std::{
  cell::RefCell,
  fs::{self},
  io::{Cursor, Read, Seek},
  path::{Component, Path, PathBuf},
};

use crate::{
  error::Error,
  lsb::read::LsbReadExt,
  lsf::read::LsfReadExt,
  lsv::package::{Package, PackageHandle, Transform},
  resource::Resource,
};

pub enum File {
  Cursor(Cursor<Vec<u8>>),
  FsFile(fs::File),
}

impl Read for File {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    match self {
      Self::Cursor(reader) => reader.read(buf),
      Self::FsFile(reader) => reader.read(buf),
    }
  }
}

impl Seek for File {
  fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
    match self {
      Self::Cursor(reader) => reader.seek(pos),
      Self::FsFile(reader) => reader.seek(pos),
    }
  }
}

pub enum FileReference<I> {
  LsvFile(RefCell<PackageHandle<I>>, PathBuf),
  OsFile(PathBuf),
}

impl<I: Read + Seek> FileReference<I> {
  pub fn as_lsv(&mut self) -> Result<PackageHandle<File>, Error> {
    match self {
      Self::LsvFile(handle, path) => {
        let reader = PackageHandle::open(path, |path| {
          let mut handle = handle.borrow_mut();
          let contents = handle.file_contents(path)?;
          Ok(File::Cursor(Cursor::new(contents.clone())))
        })?;
        Ok(reader)
      }
      Self::OsFile(path) => Ok(PackageHandle::open(path, |path| {
        Ok(File::FsFile(fs::File::open(path)?))
      })?),
    }
  }

  pub fn as_lsf(&mut self) -> Result<Resource, Error> {
    match self {
      Self::LsvFile(handle, path) => {
        let mut handle = handle.borrow_mut();
        let resource = handle.lsf_file(path)?;
        Ok(resource)
      }
      Self::OsFile(path) => {
        let mut reader = fs::File::open(path)?;
        Ok(reader.read_lsf_resource()?)
      }
    }
  }

  pub fn as_lsb(&mut self) -> Result<Resource, Error> {
    match self {
      Self::LsvFile(handle, path) => {
        let mut handle = handle.borrow_mut();
        let resource = handle.lsb_file(path)?;
        Ok(resource)
      }
      Self::OsFile(path) => {
        let mut reader = fs::File::open(path)?;
        Ok(reader.read_lsb_resource()?)
      }
    }
  }
}

impl File {
  pub fn open(path: &Path) -> Result<FileReference<fs::File>, Error> {
    let mut lsv_path_components: Option<Vec<Component>> = None;
    let mut file_path_components: Vec<Component> = vec![];
    let path_components: Vec<Component> = path.components().collect();
    let last_index = path_components.len() - 1;
    for (index, &component) in path_components.iter().enumerate() {
      file_path_components.push(component);

      if component.as_os_str().to_str().unwrap().ends_with(".lsv") && index != last_index {
        lsv_path_components = Some(file_path_components.clone());
        file_path_components.clear();
      }
    }

    let file_path = file_path_components
      .iter()
      .fold(PathBuf::new(), |mut path, component| {
        path.push(component);
        path
      });

    if let Some(components) = lsv_path_components {
      let lsv_path = components
        .iter()
        .fold(PathBuf::new(), |mut path, component| {
          path.push(component);
          path
        });
      let handle = Package::open_file(&lsv_path)?;
      return Ok(FileReference::LsvFile(RefCell::new(handle), file_path));
    }
    Ok(FileReference::OsFile(file_path))
  }
}
