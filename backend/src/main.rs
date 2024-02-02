mod api;
mod db;

use crate::api::AppState;
use actix_web::{web, App, HttpServer};
use anyhow::Result;

#[actix_web::main]
#[allow(clippy::expect_used)]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    HttpServer::new(|| {
        let pool = match db::new_connection_pool() {
            Ok(pool) => pool,
            Err(err) => panic!("Failed to create connection pool: {err}"),
        };

        App::new()
            .app_data(web::Data::new(AppState::new(pool)))
            .service(api::routes::get_all_todos)
            .service(api::routes::get_todo_by_id)
            .service(api::routes::get_todos_by_status)
            .service(api::routes::create_todo)
            .service(api::routes::update_todo)
            .service(api::routes::delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
