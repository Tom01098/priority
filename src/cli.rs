use crate::db::NewTodo;
use crate::{db, schema};
use clap::Parser;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub fn handle(&self, connection: &mut SqliteConnection) {
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
    fn handle(&self, connection: &mut SqliteConnection) {
        let new_todo = NewTodo::new(&self.title);
        diesel::insert_into(schema::todo::table)
            .values(&new_todo)
            .execute(connection)
            .unwrap();
    }
}

fn handle_list(connection: &mut SqliteConnection) {
    let todos = schema::todo::dsl::todo
        .select(db::Todo::as_select())
        .load(connection)
        .unwrap();

    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["ID", "Title"]);

    for todo in todos.iter() {
        builder.push_record(todo.as_row());
    }

    let table = builder.build();
    println!("{table}");
}
