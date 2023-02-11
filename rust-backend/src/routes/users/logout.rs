use crate::cookie;
use crate::database::db;
use crate::database::db::Db;
use crate::models::GenericResponse;
use crate::response_helper::{bad_request_response, internal_server_error_response};
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    post, web, HttpRequest, HttpResponse, Responder,
};
use futures::lock::Mutex;

#[post("/users/logout")]
pub async fn logout(db: web::Data<Mutex<Db>>, req: HttpRequest) -> impl Responder {
    let cookie = match cookie::from_header(req.headers()) {
        Ok(cookie) => cookie.value().to_string(),
        Err(err) => {
            return bad_request_response(match err {
                cookie::Error::Malformed => "malformed cookie header".to_string(),
                cookie::Error::NotIncluded => "cookie header not included".to_string(),
            })
        }
    };

    let mut db = (**db).lock().await;
    match db.session_user_from_token(cookie.clone()).await {
        Ok(_) => {}
        Err(db::Error::NotFound) => return bad_request_response("invalid cookie".to_string()),
        Err(_) => return internal_server_error_response("db error".to_string()),
    };

    if db.delete_user_session(cookie).await.is_err() {
        return internal_server_error_response("db error".to_string());
    };

    let removal_cookie = Cookie::build("SESSION_TOKEN", "")
        .max_age(Duration::new(0, 0))
        .finish();

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .cookie(removal_cookie)
        .json(GenericResponse {
            ok: true,
            msg: "success".to_string(),
        })
}
