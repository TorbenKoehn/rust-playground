mod list;
mod show;

use std::path::PathBuf;

use clap::Subcommand;

use crate::{error::Error, util::output::OutputFormat};

use self::{list::cli_lsf_list, show::cli_lsf_show};

#[derive(Subcommand, Debug)]
pub enum LsfCommand {
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

pub async fn cli_lsf(command: LsfCommand) -> Result<(), Error> {
  match command {
    LsfCommand::List { path } => cli_lsf_list(path).await,
    LsfCommand::Show {
      path,
      output_format,
    } => cli_lsf_show(path, output_format).await,
  }
}
