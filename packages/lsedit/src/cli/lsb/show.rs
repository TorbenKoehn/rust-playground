use std::path::PathBuf;

use lslib::file::File;

use crate::{error::Error, util::output::OutputFormat};

pub async fn cli_lsb_show(path: PathBuf, output_format: Option<OutputFormat>) -> Result<(), Error> {
  let file = File::open(&path)?.as_lsb()?;
  let format = output_format.unwrap_or(OutputFormat::Structure);

  match format {
    OutputFormat::Structure => {
      println!("{:#?}", file);
    }
    OutputFormat::Json => {
      println!("{}", serde_json::to_string_pretty(&file)?);
    }
    OutputFormat::Yaml => {
      println!("{}", serde_yaml::to_string(&file)?);
    }
    OutputFormat::Xml => {
      println!("{}", quick_xml::se::to_string(&file)?);
    }
  }

  Ok(())
}
