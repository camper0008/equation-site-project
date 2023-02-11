use crate::models::GenericResponse;

use actix_web::{http::header::ContentType, HttpResponse};

pub fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(GenericResponse { ok: false, msg })
}

pub fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(GenericResponse { ok: false, msg })
}
