mod cli;
mod db;
mod error;
mod table;

use crate::cli::Cli;
use crate::db::connect;
use crate::error::Result;
use clap::Parser;
use dotenvy::dotenv;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();

    dotenv().ok();

    let url = args.database_url()?;
    let mut connection = connect(&url)?;

    args.handle(&mut connection)?;

    Ok(())
}
