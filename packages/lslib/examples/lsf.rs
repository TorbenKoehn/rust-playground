use std::path::Path;

use lslib::{
  error::Error,
  lsv::package::{Package, Transform},
};
use serde_yaml::to_string;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let mut package = Package::open_file(&Path::new("examples/example.lsv"))?;

  println!("{:#?}", package);

  let lsf = package.lsf_file(&Path::new("levelcache/charactercreation.lsf"))?;
  let yaml_string = to_string(&lsf)?;
  println!("{}", yaml_string);

  Ok(())
}
