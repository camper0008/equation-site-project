use crate::database::db::Db;
use crate::models::User;
use crate::utils::{bad_request_response, internal_server_error_response};
use actix_web::{
    cookie::Cookie, get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct InfoResponse {
    ok: bool,
    msg: String,
    user: User,
}

#[get("/users/info")]
pub async fn info(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
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

    let db_user = found.unwrap();

    let user = User {
        id: db_user.id,
        username: db_user.username,
        permission: db_user.permission,
        date_created: db_user.date_created,
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(InfoResponse {
            ok: true,
            msg: "success".to_string(),
            user: user,
        })
}
