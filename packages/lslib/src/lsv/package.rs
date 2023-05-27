use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek, Write};
use std::path::PathBuf;
use std::{fmt::Formatter, path::Path};

use std::fs;

use crate::error::Error;
use crate::lsb::read::LsbReadExt;
use crate::lsf::read::LsfReadExt;
use crate::resource::Resource;

use super::file::{File, FileTable};
use super::header::Header;
use super::read::{read_lsv_file_contents, LsvReadExt};

pub type FileMap = HashMap<String, File>;

#[derive(Default)]
pub struct Package {
  header: Header,
  file_table: FileTable,
}

impl Package {
  pub fn open_file(path: &Path) -> Result<PackageHandle<fs::File>, Error> {
    PackageHandle::open(path, |path| Ok(fs::File::open(path)?))
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn header_mut(&mut self) -> &mut Header {
    &mut self.header
  }

  pub fn file_table(&self) -> &FileTable {
    &self.file_table
  }

  pub fn file_table_mut(&mut self) -> &mut FileTable {
    &mut self.file_table
  }

  pub fn files(&self) -> &HashMap<PathBuf, File> {
    self.file_table.files()
  }

  pub fn files_mut(&mut self) -> &mut HashMap<PathBuf, File> {
    self.file_table.files_mut()
  }

  pub fn file(&self, path: &Path) -> Option<&File> {
    self.file_table.file(path)
  }

  pub fn file_mut(&mut self, path: &Path) -> Option<&mut File> {
    self.file_table.file_mut(path)
  }
}

impl Debug for Package {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Package")
      .field("header", &self.header)
      .field("files", &self.file_table.files())
      .finish()
  }
}

pub struct PackageHandle<I> {
  package: Package,
  path: PathBuf,
  streams: Vec<I>,
}

fn part_path(path: &Path, part: u16) -> PathBuf {
  path.parent().unwrap().join(format!(
    "{}_{}{}",
    path.file_name().unwrap().to_str().unwrap(),
    part,
    path.extension().unwrap().to_str().unwrap()
  ))
}

fn part_paths(path: &Path, size: u16) -> Vec<PathBuf> {
  let mut part_files: Vec<PathBuf> = Vec::with_capacity(size as usize);
  for index in 0..size {
    part_files.push(part_path(path, index))
  }
  part_files
}

impl<I> PackageHandle<I> {
  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn package(&self) -> &Package {
    &self.package
  }

  pub fn package_mut(&mut self) -> &mut Package {
    &mut self.package
  }

  pub fn header(&self) -> &Header {
    &self.package.header
  }

  pub fn header_mut(&mut self) -> &mut Header {
    &mut self.package.header
  }

  pub fn file_table(&self) -> &FileTable {
    &self.package.file_table
  }

  pub fn file_table_mut(&mut self) -> &mut FileTable {
    &mut self.package.file_table
  }

  pub fn files(&self) -> &HashMap<PathBuf, File> {
    self.package.file_table.files()
  }

  pub fn files_mut(&mut self) -> &mut HashMap<PathBuf, File> {
    self.package.file_table.files_mut()
  }

  pub fn file(&self, path: &Path) -> Option<&File> {
    self.package.file_table.file(path)
  }

  pub fn file_mut(&mut self, path: &Path) -> Option<&mut File> {
    self.package.file_table.file_mut(path)
  }
}

impl<I> Debug for PackageHandle<I> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{:?}", self.path))
  }
}

impl<I: Read + Seek> PackageHandle<I> {
  pub fn open<F: Fn(&Path) -> Result<I, Error>>(
    path: &Path,
    create_reader: F,
  ) -> Result<PackageHandle<I>, Error> {
    let mut package = Package::default();
    let dir = path
      .parent()
      .ok_or(Error::InvalidPath(path.to_str().unwrap().to_owned()))?;
    fs::create_dir_all(dir)?;
    let mut main_buffer = create_reader(path)?;
    main_buffer.read_lsv_package(&mut package)?;
    let header = package.header();

    // Create all buffers, including opening all related part files
    let mut streams: Vec<I> = Vec::with_capacity(header.part_count() as usize);
    let package_part_paths = part_paths(&path, header.part_count() - 1);
    streams.insert(0, main_buffer);
    for (index, part_path) in package_part_paths.iter().enumerate() {
      let part_buffer = create_reader(part_path)?;
      streams.insert(index + 1, part_buffer);
    }

    let package = PackageHandle {
      path: path.to_owned(),
      package,
      streams,
    };
    Ok(package)
  }

  pub fn file_contents(&mut self, path: &Path) -> Result<&Vec<u8>, Error> {
    if self.package.file_table.file(path).is_none() {
      return Err(Error::FileNotFound(path.to_str().unwrap().to_owned()));
    }

    if let None = self.package.file_table.file(path).unwrap().contents() {
      let file = self.package.file_table.file_mut(path).unwrap();
      let contents = read_lsv_file_contents(&mut self.streams, &self.package.header, &file)?;
      file.set_contents(Some(contents));
    }

    if let Some(contents) = self.package.file_table.file(path).unwrap().contents() {
      Ok(contents)
    } else {
      Err(Error::InvalidFileTable)
    }
  }
}

pub trait Unpack {
  fn unpack(&mut self, target_dir: &Path) -> Result<(), Error>;
}

impl<I: Read + Seek> Unpack for PackageHandle<I> {
  fn unpack(&mut self, target_dir: &Path) -> Result<(), Error> {
    let file_paths: Vec<PathBuf> = self.package.files().keys().cloned().collect();
    for file_path in file_paths {
      let target_path = target_dir.join(&file_path);
      let contents = self.file_contents(&file_path)?;
      let file_dir = target_path
        .parent()
        .ok_or(Error::InvalidPath(target_path.to_str().unwrap().to_owned()))?;
      fs::create_dir_all(file_dir)?;
      let mut file_handle = fs::File::create(target_path)?;
      file_handle.write_all(contents)?;
    }
    Ok(())
  }
}

pub trait Transform {
  fn lsf_file(&mut self, path: &Path) -> Result<Resource, Error>;
  fn lsb_file(&mut self, path: &Path) -> Result<Resource, Error>;
}

impl<I: Read + Seek> Transform for PackageHandle<I> {
  fn lsf_file(&mut self, path: &Path) -> Result<Resource, Error> {
    let contents = self.file_contents(path)?;
    let mut cursor = Cursor::new(contents);
    Ok(cursor.read_lsf_resource()?)
  }

  fn lsb_file(&mut self, path: &Path) -> Result<Resource, Error> {
    let contents = self.file_contents(path)?;
    let mut cursor = Cursor::new(contents);
    Ok(cursor.read_lsb_resource()?)
  }
}
