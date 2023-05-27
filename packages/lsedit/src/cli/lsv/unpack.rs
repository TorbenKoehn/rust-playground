use std::path::PathBuf;

use lslib::{file::File, lsv::package::Unpack};

use crate::error::Error;
pub async fn cli_lsv_unpack(path: PathBuf, target_dir: PathBuf) -> Result<(), Error> {
  let mut builder = File::open(&path)?.as_lsv()?;
  builder.unpack(&target_dir)?;
  Ok(())
}
