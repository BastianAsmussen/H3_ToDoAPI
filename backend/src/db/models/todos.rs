use anyhow::Result;
use diesel::{
    AsChangeset, ExpressionMethods, Insertable, OptionalExtension, PgConnection, QueryDsl,
    Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::db::schema::todos;

/// A todo item.
///
/// # Fields
///
/// * `id` - The unique identifier for the todo.
///
/// * `title` - The title of the todo.
/// * `completed` - Whether the todo is completed.
#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,

    pub title: String,
    pub completed: bool,
}

impl Todo {
    /// Get all todos from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Todo>>` - The list of todos if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error getting the todos from the database.
    pub fn all(conn: &mut PgConnection) -> Result<Vec<Self>> {
        let todos = todos::table.select(Self::as_select()).load(conn)?;

        Ok(todos)
    }

    /// Get a todo by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    /// * `id` - The unique identifier for the todo.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Todo>>` - The todo if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error getting the todo from the database.
    pub fn by_id(conn: &mut PgConnection, id: i32) -> Result<Option<Self>> {
        let todo = todos::table
            .filter(todos::id.eq(id))
            .select(Self::as_select())
            .first(conn)
            .optional()?;

        Ok(todo)
    }

    /// Get all completed todos from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    /// * `completed` - Whether the todo is completed.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Todo>>` - The list of completed todos if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error getting the completed todos from the database.
    pub fn by_status(conn: &mut PgConnection, completed: bool) -> Result<Vec<Self>> {
        let todos = todos::table
            .filter(todos::completed.eq(completed))
            .select(Self::as_select())
            .load(conn)?;

        Ok(todos)
    }

    /// Delete a todo by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    /// * `id` - The unique identifier for the todo.
    ///
    /// # Returns
    ///
    /// * `Result<usize>` - The number of todos deleted if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error deleting the todo from the database.
    pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize> {
        let deleted = diesel::delete(todos::table.filter(todos::id.eq(id))).execute(conn)?;

        Ok(deleted)
    }
}

/// A new todo item.
///
/// # Fields
///
/// * `title` - The title of the todo.
/// * `completed` - Whether the todo is completed.
#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTodo {
    pub title: String,
    pub completed: Option<bool>,
}

impl NewTodo {
    /// Insert a new todo into the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// * `Result<Todo>` - The new todo if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error inserting the todo into the database.
    pub fn insert(&self, conn: &mut PgConnection) -> Result<Todo> {
        let todo = diesel::insert_into(todos::table)
            .values(self)
            .get_result(conn)?;

        Ok(todo)
    }

    /// Update a todo in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    /// * `id` - The unique identifier for the todo.
    ///
    /// # Returns
    ///
    /// * `Result<Todo>` - The updated todo if successful.
    ///
    /// # Errors
    ///
    /// * If there is an error updating the todo in the database.
    pub fn update(&self, conn: &mut PgConnection, id: i32) -> Result<Todo> {
        let todo = diesel::update(todos::table)
            .filter(todos::id.eq(id))
            .set(self)
            .get_result(conn)?;

        Ok(todo)
    }
}
