use actix_web::{App, HttpServer};

mod database;
mod models;
mod routes;

use crate::routes::users;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(users::login::login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
