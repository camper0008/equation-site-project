use crate::database::db::{Db, DbError};
use crate::models::PreviewableEquation;
use crate::utils::internal_server_error_response;
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct SearchResponse {
    ok: bool,
    msg: String,
    equations: Vec<PreviewableEquation>,
}

#[get("/equations/search/{title}")]
pub async fn search(db: web::Data<Mutex<Db>>, title: web::Path<String>) -> impl Responder {
    let db_result = (**db)
        .lock()
        .unwrap()
        .equation_search(title.to_string())
        .await;

    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found_equations = db_result.ok().unwrap();

    let previewable_equations = found_equations
        .into_iter()
        .map(|eq| PreviewableEquation {
            id: eq.id,
            title: eq.title,
            date_created: eq.date_created,
        })
        .collect();

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(SearchResponse {
            ok: true,
            msg: "success".to_string(),
            equations: previewable_equations,
        })
}
