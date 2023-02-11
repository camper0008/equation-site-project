use crate::database::db::Db;
use crate::models::PreviewableEquation;
use crate::search::equations;
use crate::utils::internal_server_error_response;
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

    let equations_result = equations(&mut db, title.to_string()).await;

    if equations_result.is_err() {
        return internal_server_error_response(format!(
            "db error: {:?}",
            equations_result.err().unwrap()
        ));
    }

    let equations = equations_result.ok().unwrap();

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(SearchResponse {
            ok: true,
            msg: "success".to_string(),
            equations,
        })
}
