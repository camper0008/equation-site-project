use crate::database::db::DbParam;
use crate::database::mongo_db::MongoDb;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use futures::lock::Mutex;
use std::env;
use std::sync::Arc;

mod char_generation;
mod cookie;
mod database;
mod date_helper;
mod models;
mod response_helper;
mod routes;
mod search;

use crate::routes::equations;
use crate::routes::users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("unable to load .env file");

    let uri = env::var("MONGO_URI").expect("unable to get MONGO_URI environment variable");
    println!("Attempting to connect to mongodb...");
    let db = MongoDb::new(uri.to_string(), "equation-site-project".to_string()).await;
    println!("MongoDB connected");

    let db: Arc<DbParam> = Arc::new(Mutex::new(db));
    let db: Data<DbParam> = Data::from(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            // TODO: set proper cors settings
            .wrap(Cors::permissive())
            .service(users::login::login)
            .service(users::logout::logout)
            .service(users::create::create)
            .service(users::info::info)
            .service(equations::create::create)
            .service(equations::search::search)
            .service(equations::one::one)
            .service(equations::edit::edit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
