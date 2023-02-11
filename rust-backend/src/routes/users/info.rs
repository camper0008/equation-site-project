use crate::cookie::CookieHeaderError;
use crate::models::User;
use crate::response_helper::{bad_request_response, internal_server_error_response};
use crate::{cookie::cookie_from_header, database::db::Db};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder};
use futures::lock::Mutex;
use serde::Serialize;

#[derive(Serialize)]
struct InfoResponse {
    ok: bool,
    msg: String,
    user: User,
}

#[get("/users/info")]
pub async fn info(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
    let cookie_result = cookie_from_header(req.headers());
    if cookie_result.is_err() {
        return bad_request_response(match cookie_result.err().unwrap() {
            CookieHeaderError::Malformed => "malformed cookie header".to_string(),
            CookieHeaderError::NotIncluded => "cookie header not included".to_string(),
        });
    }
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
            user,
        })
}
