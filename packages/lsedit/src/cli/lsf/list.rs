use std::path::PathBuf;

use lslib::{file::File, resource::reader::ResourceReader, util::arena::ArenaReader};

use crate::error::Error;

pub async fn cli_lsf_list(path: PathBuf) -> Result<(), Error> {
  let file = File::open(&path)?.as_lsf()?;

  file.root_indexes().iter().for_each(|root_index| {
    file.recursive_iter(*root_index).for_each(|index| {
      file
        .value(index)
        .attributes()
        .iter()
        .for_each(|(key, value)| {
          println!("{}/{} = {:?}", file.full_path(index), key, value);
        });
    })
  });

  Ok(())
}
