mod db;
mod schema;

use crate::db::NewTodo;
use clap::Parser;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Add(Add),
}

#[derive(Debug, Parser)]
struct Add {
    title: String,
}

fn main() {
    let args = Cli::parse();

    dotenv().ok();

    let mut connection = establish_connection();
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    match args.command {
        Command::Add(add) => handle_add(&mut connection, &add),
    }
}

fn establish_connection() -> SqliteConnection {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        format!(
            "sqlite://{}/.priority",
            std::env::home_dir().unwrap().display()
        )
    });
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error opening {url}"))
}

fn handle_add(connection: &mut SqliteConnection, add: &Add) {
    let new_todo = NewTodo { title: &add.title };
    diesel::insert_into(schema::todo::table)
        .values(&new_todo)
        .execute(connection)
        .unwrap();
}
