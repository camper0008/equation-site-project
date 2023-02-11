use crate::database::db::Error;
use crate::models::Equation;
use crate::response_helper::bad_request_response;
use crate::{database::db::Db, response_helper::internal_server_error_response};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use futures::lock::Mutex;
use serde::Serialize;

#[derive(Serialize)]
struct OneResponse {
    ok: bool,
    msg: String,
    equation: Equation,
}

#[get("/equations/one/{post_id}")]
pub async fn one(db: web::Data<Mutex<Db>>, post_id: web::Path<String>) -> impl Responder {
    let db = (**db).lock().await;

    let equation = match db.equation_from_id(post_id.to_string()).await {
        Ok(equation) => equation,
        Err(Error::NotFound) => {
            return bad_request_response("invalid id".to_string());
        }
        Err(_) => {
            return internal_server_error_response("db error".to_string());
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
            msg: "success".to_string(),
            equation,
        })
}
