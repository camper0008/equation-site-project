use crate::cookie::CookieHeaderError;
use crate::database::db::Error;
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
    let cookie = match cookie_from_header(req.headers()) {
        Ok(cookie) => cookie.value().to_string(),
        Err(err) => {
            return bad_request_response(match err {
                CookieHeaderError::Malformed => "malformed cookie header".to_string(),
                CookieHeaderError::NotIncluded => "cookie header not included".to_string(),
            });
        }
    };

    let mut db = (**db).lock().await;

    let user = match db.session_user_from_token(cookie).await {
        Ok(user) => user,
        Err(Error::NotFound) => {
            return bad_request_response("invalid cookie".to_string());
        }
        Err(_) => {
            return internal_server_error_response("db error".to_string());
        }
    };

    let user = User {
        id: user.id,
        username: user.username,
        permission: user.permission,
        date_created: user.date_created,
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(InfoResponse {
            ok: true,
            msg: "success".to_string(),
            user,
        })
}
