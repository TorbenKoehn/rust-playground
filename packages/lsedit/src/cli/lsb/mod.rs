mod list;
mod show;

use std::path::PathBuf;

use clap::Subcommand;

use crate::{error::Error, util::output::OutputFormat};

use self::{list::cli_lsb_list, show::cli_lsb_show};

#[derive(Subcommand, Debug)]
pub enum LsbCommand {
  List {
    #[arg(index = 1)]
    path: PathBuf,
  },
  Show {
    #[arg(index = 1)]
    path: PathBuf,
    #[arg(short, long)]
    output_format: Option<OutputFormat>,
  },
}

pub async fn cli_lsb(command: LsbCommand) -> Result<(), Error> {
  match command {
    LsbCommand::List { path } => cli_lsb_list(path).await,
    LsbCommand::Show {
      path,
      output_format,
    } => cli_lsb_show(path, output_format).await,
  }
}
