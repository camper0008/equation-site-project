use crate::database::mongo_db::MongoDb;
use actix_web::{web::Data, App, HttpServer};
use std::sync::Mutex;

mod database;
mod models;
mod routes;

use crate::routes::users;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = MongoDb::new(
        "mongodb://localhost:27017/".to_string(),
        "equation-site-project".to_string(),
    )
    .await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Mutex::new(db.clone())))
            .service(users::login::login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
