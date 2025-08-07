use crate::db;
use crate::error::Result;
use clap::Parser;
use db::Todo;
use diesel::{RunQueryDsl, SqliteConnection};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub fn handle(&self, connection: &mut SqliteConnection) -> Result<()> {
        match &self.command {
            Command::Add(add) => add.handle(connection),
            Command::List => handle_list(connection),
        }
    }
}

#[derive(Debug, Parser)]
pub enum Command {
    Add(Add),
    List,
}

#[derive(Debug, Parser)]
pub struct Add {
    title: String,
}

impl Add {
    fn handle(&self, connection: &mut SqliteConnection) -> Result<()> {
        let create_query = Todo::create(&self.title);
        create_query.execute(connection)?;
        Ok(())
    }
}

fn handle_list(connection: &mut SqliteConnection) -> Result<()> {
    let todos = Todo::list().load(connection)?;

    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["ID", "Title"]);

    for todo in todos.iter() {
        builder.push_record(todo.as_row());
    }

    let table = builder.build();
    println!("{table}");
    Ok(())
}
