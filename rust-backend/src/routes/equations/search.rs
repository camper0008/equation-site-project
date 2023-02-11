use crate::models::PreviewableEquation;
use crate::search::equations;
use crate::{database::db::Db, response_helper::internal_server_error_response};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use futures::lock::Mutex;
use serde::Serialize;

#[derive(Serialize)]
struct SearchResponse {
    ok: bool,
    msg: String,
    equations: Vec<PreviewableEquation>,
}

#[get("/equations/search/{title}")]
pub async fn search(db: web::Data<Mutex<Db>>, title: web::Path<String>) -> impl Responder {
    let mut db = (**db).lock().await;

    let Ok(equations) = equations(&mut db, title.to_string()).await else {
        return internal_server_error_response("db error".to_string());
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(SearchResponse {
            ok: true,
            msg: "success".to_string(),
            equations,
        })
}
