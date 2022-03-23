use crate::database::db::Db;
use crate::models::Equation;
use crate::utils::{bad_request_response, internal_server_error_response};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct OneResponse {
    ok: bool,
    msg: String,
    equation: Equation,
}

#[get("/equations/one/{post_id}")]
pub async fn one(db: web::Data<Mutex<Db>>, post_id: web::Path<String>) -> impl Responder {
    let db_result = (**db)
        .lock()
        .unwrap()
        .equation_from_id(post_id.to_string())
        .await;

    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found_equation_option = db_result.ok().unwrap();
    if found_equation_option.is_none() {
        return bad_request_response("invalid id".to_string());
    }

    let found_equation = found_equation_option.unwrap();
    let equation = Equation {
        id: found_equation.id.clone(),
        title: found_equation.title.clone(),
        content: found_equation.content.clone(),
        date_created: found_equation.date_created.clone(),
        creator_id: found_equation.creator_id.clone(),
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(OneResponse {
            ok: true,
            msg: "success".to_string(),
            equation: equation,
        })
}
