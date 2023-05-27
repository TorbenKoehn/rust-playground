use std::path::Path;

use lslib::{error::Error, lsv::package::Package};

fn main() -> Result<(), Error> {
  let package = Package::open_file(&Path::new("examples/example.lsv"))?;
  println!("{:#?}", package);
  Ok(())
}
