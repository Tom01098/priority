mod schema;

use diesel::dsl::{AsSelect, Select};
use diesel::prelude::*;
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::sqlite::Sqlite;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::error::{DatabaseError, Error, Result};

pub fn connect(url: &str) -> Result<SqliteConnection> {
    let mut connection = SqliteConnection::establish(url).map_err(|source| {
        Error::Database(DatabaseError::Connection {
            url: url.to_string(),
            source,
        })
    })?;
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|source| {
            Error::Database(DatabaseError::Migration {
                url: url.to_string(),
                source,
            })
        })?;
    Ok(connection)
}

#[derive(Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = schema::todo)]
#[diesel(check_for_backend(Sqlite))]
pub struct Todo {
    id: i32,
    title: String,
}

pub type AllTodos = Select<schema::todo::table, AsSelect<Todo, Sqlite>>;

impl Todo {
    pub fn as_row(&self) -> Vec<String> {
        vec![self.id.to_string(), self.title.clone()]
    }

    pub fn list() -> AllTodos {
        use schema::todo::dsl::*;
        todo.select(Todo::as_select())
    }

    pub fn create<'a>(
        title: &'a str,
    ) -> impl RunQueryDsl<SqliteConnection> + QueryId + QueryFragment<Sqlite> + 'a {
        let new_todo = NewTodo::new(title);
        diesel::insert_into(schema::todo::table)
            .values(new_todo)
            .returning(Todo::as_select())
    }
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = schema::todo)]
#[diesel(check_for_backend(Sqlite))]
struct NewTodo<'a> {
    title: &'a str,
}

impl<'a> NewTodo<'a> {
    fn new(title: &'a str) -> Self {
        Self { title }
    }
}
