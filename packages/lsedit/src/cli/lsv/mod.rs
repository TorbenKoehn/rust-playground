mod list;
mod unpack;

use std::path::PathBuf;

use clap::Subcommand;

use crate::error::Error;

use self::{list::cli_lsv_list, unpack::cli_lsv_unpack};

#[derive(Subcommand, Debug)]
pub enum LsvCommand {
  List {
    #[arg(index = 1)]
    path: PathBuf,
  },
  Unpack {
    #[arg(index = 1)]
    path: PathBuf,
    #[arg(index = 2)]
    target_dir: PathBuf,
  },
}

pub async fn cli_lsv(command: LsvCommand) -> Result<(), Error> {
  match command {
    LsvCommand::List { path } => cli_lsv_list(path).await,
    LsvCommand::Unpack { path, target_dir } => cli_lsv_unpack(path, target_dir).await,
  }
}
