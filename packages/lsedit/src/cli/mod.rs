mod dos_ee;
mod lsb;
mod lsf;
mod lsv;
use clap::{Parser, Subcommand};

use crate::error::Error;

use self::{
  dos_ee::{cli_dos_ee, DosEeCommand},
  lsb::{cli_lsb, LsbCommand},
  lsf::{cli_lsf, LsfCommand},
  lsv::{cli_lsv, LsvCommand},
};

#[derive(Subcommand, Debug)]
enum Command {
  #[command(subcommand)]
  Lsv(LsvCommand),
  #[command(subcommand)]
  Lsf(LsfCommand),
  #[command(subcommand)]
  Lsb(LsbCommand),

  #[command(subcommand)]
  DosEe(DosEeCommand),
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
    Command::Lsv(lsv_command) => cli_lsv(lsv_command).await,
    Command::Lsf(lsf_command) => cli_lsf(lsf_command).await,
    Command::Lsb(lsb_command) => cli_lsb(lsb_command).await,
    Command::DosEe(dos_ee_command) => cli_dos_ee(dos_ee_command).await,
  }
}
