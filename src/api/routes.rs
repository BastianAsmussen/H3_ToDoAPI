use crate::api::AppState;
use crate::db::models::todos::{NewTodo, Todo};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use std::fmt::Display;

/// The status of a todo.
///
/// # Variants
///
/// * `Complete` - The todo is complete.
/// * `Incomplete` - The todo is incomplete.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Complete,
    Incomplete,
}

impl From<Status> for bool {
    fn from(value: Status) -> Self {
        match value {
            Status::Complete => true,
            Status::Incomplete => false,
        }
    }
}

/// Return an error response.
///
/// # Arguments
///
/// * `err` - The error to return.
///
/// # Returns
///
/// * `HttpResponse` - The error response.
fn return_error<E: Display>(err: E) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!("Internal Server Error: {err}"))
}

/// Get all todos.
///
/// # Arguments
///
/// * `data` - The application state.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[get("/todos/all")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    Todo::all(&mut conn).map_or_else(return_error, |todos| HttpResponse::Ok().json(todos))
}

/// Get a todo by its unique identifier.
///
/// # Arguments
///
/// * `data` - The application state.
/// * `id` - The unique identifier for the todo.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[get("/todos/by_id/{id}")]
pub async fn get_todo_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    Todo::by_id(&mut conn, id.into_inner()).map_or_else(return_error, |todo| {
        todo.map_or_else(
            || HttpResponse::NotFound().finish(),
            |todo| HttpResponse::Ok().json(todo),
        )
    })
}

/// Get all todos by their status.
///
/// # Arguments
///
/// * `data` - The application state.
/// * `status` - The status of the todos.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[get("/todos/by_status/{status}")]
pub async fn get_todos_by_status(
    data: web::Data<AppState>,
    status: web::Path<Status>,
) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    Todo::by_status(&mut conn, status.into_inner().into())
        .map_or_else(return_error, |todos| HttpResponse::Ok().json(todos))
}

/// Create a new todo.
///
/// # Arguments
///
/// * `data` - The application state.
/// * `new_todo` - The new todo to create.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[post("/todos/new")]
pub async fn create_todo(
    data: web::Data<AppState>,
    new_todo: web::Json<NewTodo>,
) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    new_todo
        .insert(&mut conn)
        .map_or_else(return_error, |todo| HttpResponse::Created().json(todo))
}

/// Update a todo.
///
/// # Arguments
///
/// * `data` - The application state.
/// * `id` - The unique identifier for the todo.
/// * `new_todo` - The new todo to update.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[put("/todos/update/{id}")]
pub async fn update_todo(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    new_todo: web::Json<NewTodo>,
) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    new_todo
        .update(&mut conn, id.into_inner())
        .map_or_else(return_error, |todo| HttpResponse::Ok().json(todo))
}

/// Delete a todo.
///
/// # Arguments
///
/// * `data` - The application state.
/// * `id` - The unique identifier for the todo.
///
/// # Returns
///
/// * `impl Responder` - The HTTP response.
#[delete("/todos/delete/{id}")]
pub async fn delete_todo(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let mut conn = match data.connection() {
        Ok(conn) => conn,
        Err(err) => return return_error(err),
    };

    Todo::delete(&mut conn, id.into_inner())
        .map_or_else(return_error, |deleted| HttpResponse::Ok().json(deleted))
}
