mod schema;

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub fn connect(url: &str) -> SqliteConnection {
    let mut connection =
        SqliteConnection::establish(url).unwrap_or_else(|_| panic!("Error opening {url}"));
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    connection
}

#[derive(Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = schema::todo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    id: i32,
    title: String,
}

impl Todo {
    pub fn as_row(&self) -> Vec<String> {
        vec![self.id.to_string(), self.title.clone()]
    }
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = schema::todo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTodo<'a> {
    title: &'a str,
}

impl<'a> NewTodo<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }
}

pub fn create_todo<'a>(
    new_todo: &'a NewTodo<'a>,
) -> impl RunQueryDsl<SqliteConnection>
+ diesel::query_builder::QueryId
+ diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>
+ 'a {
    use schema::todo::dsl::*;
    diesel::insert_into(todo).values(new_todo)
}

pub type AllTodos =
    diesel::dsl::Select<schema::todo::table, diesel::dsl::AsSelect<Todo, diesel::sqlite::Sqlite>>;

pub fn list_todos() -> AllTodos {
    use schema::todo::dsl::*;
    todo.select(Todo::as_select())
}
