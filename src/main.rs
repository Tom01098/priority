mod cli;
mod db;
mod error;

use crate::cli::Cli;
use crate::db::connect;
use crate::error::{EnvironmentError, Error, Result};
use clap::Parser;
use dotenvy::dotenv;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::from(e.exit_code() as u8)
        }
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();

    dotenv().ok();

    let url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let home_dir =
                std::env::home_dir().ok_or(Error::Environment(EnvironmentError::HomeDir))?;
            format!("sqlite://{}/.priority", home_dir.display())
        }
    };

    let mut connection = connect(&url)?;

    args.handle(&mut connection)?;

    Ok(())
}
