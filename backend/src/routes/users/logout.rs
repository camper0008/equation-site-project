use crate::database::db::{Db, DbError};
use crate::models::GenericResponse;
use crate::utils::{bad_request_response, internal_server_error_response};
use actix_web::{
    cookie::Cookie, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder,
};
use std::sync::Mutex;

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

    let mut removal_cookie = Cookie::build("SESSION_TOKEN", "").finish();
    removal_cookie.make_removal();

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .cookie(removal_cookie)
        .json(GenericResponse {
            ok: true,
            msg: "success".to_string(),
        })
}
