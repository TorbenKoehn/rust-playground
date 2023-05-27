mod show;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::error::Error;

use self::show::cli_show;

#[derive(Subcommand, Debug)]
enum Command {
  Show {
    #[arg(short, long)]
    files: Vec<PathBuf>,
  },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Input {
  #[command(subcommand)]
  command: Command,
}

pub async fn cli_main() -> Result<(), Error> {
  let input = Input::parse();

  match input.command {
    Command::Show { files } => cli_show(files).await,
  }
}

// let mut table = Table::new();
// table
//   .load_preset(UTF8_FULL)
//   .apply_modifier(UTF8_ROUND_CORNERS)
//   .set_content_arrangement(ContentArrangement::Dynamic)
//   .set_header(vec!["Name", "Kind", "Table", "Schema"]);
// for entity in entities {
//   table.add_row(vec![name, kind, table_name, schema_name]);
// }
