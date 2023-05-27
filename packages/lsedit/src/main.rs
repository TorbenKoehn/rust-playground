mod cli;
mod error;
mod util;

use cli::cli_main;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
  cli_main().await?;

  Ok(())
}
