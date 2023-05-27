use std::path::PathBuf;

use clap::Subcommand;
use lslib::file::File;

use crate::error::Error;

use super::util::get_dos_ee_data_path;

#[derive(Subcommand, Debug)]
pub enum PlayerCommand {
  List {
    #[arg(short, long)]
    profile: String,
    #[arg(short, long)]
    save_name: String,
    #[arg(short, long)]
    data_path: Option<PathBuf>,
  },
}

pub async fn cli_dos_ee_player(command: PlayerCommand) -> Result<(), Error> {
  match command {
    PlayerCommand::List {
      profile,
      save_name,
      data_path,
    } => cli_dos_ee_player_list(profile, save_name, data_path).await,
  }?;
  Ok(())
}

async fn cli_dos_ee_player_list(
  profile: String,
  save_name: String,
  data_path: Option<PathBuf>,
) -> Result<(), Error> {
  let data_path = data_path
    .or_else(|| get_dos_ee_data_path())
    .ok_or(Error::NoDataPath)?;

  let globals_resource_path = data_path
    .join("PlayerProfiles")
    .join(profile)
    .join("Savegames_patch")
    .join(&save_name)
    .join(format!("{}.lsv", &save_name))
    .join("globals.lsf");
  println!("Loading {:?}", globals_resource_path);
  let globals_resource = File::open(&globals_resource_path)?.as_lsf()?;
  Ok(())
}
