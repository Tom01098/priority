mod cli;
mod db;
mod schema;

use crate::db::connect;
use clap::Parser;
use cli::Cli;
use dotenvy::dotenv;

fn main() {
    let args = Cli::parse();

    dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        format!(
            "sqlite://{}/.priority",
            std::env::home_dir().unwrap().display()
        )
    });
    let mut connection = connect(&url);

    args.handle(&mut connection);
}
