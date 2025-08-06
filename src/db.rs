use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub fn connect(url: &str) -> SqliteConnection {
    let mut connection =
        SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error opening {url}"));
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    connection
}

#[derive(Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = crate::schema::todo)]
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
#[diesel(table_name = crate::schema::todo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTodo<'a> {
    title: &'a str,
}

impl<'a> NewTodo<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }
}
