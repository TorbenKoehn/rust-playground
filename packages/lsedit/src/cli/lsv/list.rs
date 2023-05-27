use std::path::PathBuf;

use comfy_table::{
  modifiers::UTF8_ROUND_CORNERS,
  presets::{ASCII_HORIZONTAL_ONLY, UTF8_FULL},
  ContentArrangement, Table,
};
use lslib::file::File;

use crate::error::Error;
pub async fn cli_lsv_list(path: PathBuf) -> Result<(), Error> {
  let package = File::open(&path)?.as_lsv()?;

  let mut table = Table::new();
  table
    .load_preset(ASCII_HORIZONTAL_ONLY)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec!["Name", "Compressed", "Uncompressed"]);
  let files = package.files().values();
  for file in files {
    let cells = vec![
      file.path().to_str().unwrap().to_owned(),
      file.size_on_disk().to_string(),
      file.uncompressed_size().to_string(),
    ];
    table.add_row(cells);
  }

  println!("{}", table);

  Ok(())
}
