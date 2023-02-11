use crate::database::db::Db;
use crate::models::GenericResponse;
use crate::utils::{
    bad_request_response, cookie_from_header, internal_server_error_response, CookieHeaderError,
};
use actix_web::{
    cookie::Cookie, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder,
};
use futures::lock::Mutex;

#[post("/users/logout")]
pub async fn logout(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
    let cookie_result = cookie_from_header(req.headers());
    if cookie_result.is_err() {
        return bad_request_response(match cookie_result.err().unwrap() {
            CookieHeaderError::Malformed => "malformed cookie header".to_string(),
            CookieHeaderError::NotIncluded => "cookie header not included".to_string(),
        });
    };
    let cookie = cookie_result.ok().unwrap();

    let mut db = (**db).lock().await;
    let db_result = db.session_user_from_token(cookie.value().to_string()).await;

    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = db_result.ok().unwrap();
    if found.is_none() {
        return bad_request_response("invalid cookie".to_string());
    }

    let db_result = db.delete_user_session(cookie.value().to_string()).await;

    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
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
