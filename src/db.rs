use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todo)]
pub struct NewTodo<'a> {
    pub title: &'a str,
}
