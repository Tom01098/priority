use crate::db;
use crate::error::{EnvironmentError, Error, Result};
use crate::table::AsTable;
use clap::Parser;
use db::Todo;
use diesel::{RunQueryDsl, SqliteConnection};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(long)]
    database_url: Option<String>,
}

impl Cli {
    pub fn handle(&self, connection: &mut SqliteConnection) -> Result<()> {
        match &self.command {
            Command::Add(add) => add.handle(connection),
            Command::List(list) => list.handle(connection),
        }
    }

    pub fn database_url(&self) -> Result<String> {
        if let Some(url) = &self.database_url {
            return Ok(url.to_string());
        }

        if let Ok(url) = std::env::var("DATABASE_URL") {
            return Ok(url);
        }

        let home_dir = dirs::home_dir().ok_or(Error::Environment(EnvironmentError::HomeDir))?;
        Ok(format!("sqlite://{}/.priority", home_dir.display()))
    }
}

#[derive(Debug, Parser)]
pub enum Command {
    Add(Add),
    List(List),
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

#[derive(Debug, Parser)]
pub struct List;

impl List {
    fn handle(&self, connection: &mut SqliteConnection) -> Result<()> {
        let todos = Todo::all().load(connection)?;
        let table = todos.as_table();
        println!("{table}");
        Ok(())
    }
}
