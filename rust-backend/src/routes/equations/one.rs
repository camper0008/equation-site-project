use crate::database::db::{DbParam, Error};
use crate::models::Equation;
use crate::response_helper::bad_request_response;
use crate::response_helper::internal_server_error_response;
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct OneResponse<'a> {
    ok: bool,
    msg: &'a str,
    equation: Equation,
}

#[get("/equations/one/{post_id}")]
pub async fn one(db: web::Data<DbParam>, post_id: web::Path<String>) -> impl Responder {
    let db = (**db).lock().await;

    let equation = match db.equation_from_id(post_id.to_string()).await {
        Ok(equation) => equation,
        Err(Error::NotFound) => {
            return bad_request_response("invalid id");
        }
        Err(_) => {
            return internal_server_error_response("db error");
        }
    };

    let equation = Equation {
        id: equation.id,
        title: equation.title,
        content: equation.content,
        date_created: equation.date_created,
        creator_id: equation.creator_id,
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(OneResponse {
            ok: true,
            msg: "success",
            equation,
        })
}
