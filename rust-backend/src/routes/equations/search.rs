use crate::database::db::DbParam;
use crate::models::PreviewableEquation;
use crate::{response_helper::internal_server_error_response, search::equations};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct SearchResponse<'a> {
    ok: bool,
    msg: &'a str,
    equations: Vec<PreviewableEquation>,
}

#[get("/equations/search/{title}")]
pub async fn search(db: web::Data<DbParam>, title: web::Path<String>) -> impl Responder {
    let db = (**db).lock().await;

    let Ok(titles) = db.all_titles().await else {
        return internal_server_error_response("db error");
    };

    let equations = equations(titles, title.to_string());

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(SearchResponse {
            ok: true,
            msg: "success",
            equations,
        })
}
