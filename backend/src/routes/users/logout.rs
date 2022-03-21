use crate::database::db_trait::{Db, DbError};
use actix_web::{
    cookie::Cookie, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
pub struct LogoutResponse {
    ok: bool,
    msg: String,
}

fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(LogoutResponse {
            ok: false,
            msg: msg,
        })
}

fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(LogoutResponse {
            ok: false,
            msg: msg,
        })
}

#[post("/users/logout")]
pub async fn logout(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
    let cookie_header_option = req.headers().get("Cookie");
    if cookie_header_option.is_none() {
        return bad_request_response("no cookie header included".to_string());
    }

    let cookie_header_stringify_result = cookie_header_option.unwrap().to_str();
    if cookie_header_stringify_result.is_err() {
        return bad_request_response("malformed cookie header".to_string());
    }

    let cookie_parse_result = Cookie::parse(cookie_header_stringify_result.unwrap());
    if cookie_parse_result.is_err() {
        return bad_request_response("malformed cookie header".to_string());
    };

    let cookie = cookie_parse_result.unwrap();

    let db_result = (**db)
        .lock()
        .unwrap()
        .get_session_user_from_token(cookie.value().to_string())
        .await;

    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = db_result.unwrap();
    if found.is_none() {
        return bad_request_response("invalid cookie".to_string());
    }

    let db_result = (**db)
        .lock()
        .unwrap()
        .delete_user_session(cookie.value().to_string())
        .await;

    if db_result.is_err() {
        return internal_server_error_response(
            (match db_result {
                Ok(_) => "",
                Err(err) => match err {
                    DbError::NotFound => "db error: cookie does not exist",
                    _ => "db error",
                },
            })
            .to_string(),
        );
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(LogoutResponse {
            ok: true,
            msg: "success".to_string(),
        })
}