use crate::database::db::Db;
use crate::utils::{
    bad_request_response, get_cookie_from_header, internal_server_error_response, CookieHeaderError,
};
use actix_web::{post, web, HttpRequest, Responder};
//use serde::Deserialize;
use std::sync::Mutex;

/*
#[derive(Deserialize)]
pub struct CreateRequest {
    username: String,
    password: String,
}
*/

#[post("/equations/create")]
pub async fn create(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
    let cookie_result = get_cookie_from_header(req.headers());
    if cookie_result.is_err() {
        return bad_request_response(match cookie_result.err().unwrap() {
            CookieHeaderError::Malformed => "malformed cookie header".to_string(),
            CookieHeaderError::NotIncluded => "cookie header not included".to_string(),
        });
    };
    let cookie = cookie_result.unwrap();

    internal_server_error_response("not implemented".to_string())
}
