mod manage;
mod player;
mod util;

use std::path::PathBuf;

use clap::Subcommand;

use crate::error::Error;

use self::player::{cli_dos_ee_player, PlayerCommand};

#[derive(Subcommand, Debug)]
pub enum DosEeCommand {
  Manage {
    #[arg(short, long)]
    profile: String,
    #[arg(short, long)]
    save_name: String,
    #[arg(short, long)]
    data_path: Option<PathBuf>,
  },
  Player {
    #[command(subcommand)]
    command: PlayerCommand,
  },
}

pub async fn cli_dos_ee(command: DosEeCommand) -> Result<(), Error> {
  match command {
    DosEeCommand::Manage {
      profile,
      save_name,
      data_path,
    } => manage::cli_dos_ee_manage(profile, save_name, data_path).await,
    DosEeCommand::Player { command } => cli_dos_ee_player(command).await,
  }
}
