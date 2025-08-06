mod db;
mod schema;

use crate::db::NewTodo;
use clap::Parser;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
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
    List,
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
        Command::List => handle_list(&mut connection),
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

fn handle_list(connection: &mut SqliteConnection) {
    let todos = schema::todo::dsl::todo
        .select(db::Todo::as_select())
        .load(connection)
        .unwrap();

    let mut builder = tabled::builder::Builder::default();
    builder.insert_record(0, ["ID", "Title"]);

    for (i, todo) in todos.iter().enumerate() {
        builder.insert_record(i + 1, [&todo.id.to_string(), &todo.title]);
    }

    let table = builder.build();
    println!("{table}");
}
