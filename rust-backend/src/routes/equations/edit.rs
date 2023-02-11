use crate::cookie::{cookie_from_header, CookieHeaderError};
use crate::database::db::{Db, Error};
use crate::models::{GenericResponse, InsertableDbEquation, Permission};
use crate::response_helper::{bad_request_response, internal_server_error_response};
use actix_web::{http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder};
use futures::lock::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    title: String,
    content: String,
}

#[post("/equations/edit/{post_id}")]
pub async fn edit(
    db: web::Data<Mutex<Db>>,
    req: HttpRequest,
    body: web::Json<Request>,
    post_id: web::Path<String>,
) -> impl Responder {
    let cookie_result = cookie_from_header(req.headers());
    if cookie_result.is_err() {
        return bad_request_response(match cookie_result.err().unwrap() {
            CookieHeaderError::Malformed => "malformed cookie header".to_string(),
            CookieHeaderError::NotIncluded => "cookie header not included".to_string(),
        });
    };
    let cookie = cookie_result.ok().unwrap();

    let mut db = (**db).lock().await;

    let user_get_result = db.session_user_from_token(cookie.value().to_string()).await;

    if user_get_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found_user = user_get_result.ok().unwrap();
    if found_user.is_none() {
        return bad_request_response("invalid cookie".to_string());
    }

    let user = found_user.unwrap();

    if user.permission != Permission::Contributor && user.permission != Permission::Root {
        return bad_request_response("unauthorized".to_string());
    };

    let equation = InsertableDbEquation {
        title: body.title.clone(),
        content: body.content.clone(),
        creator_id: user.id,
    };

    match db
        .update_equation_from_id(equation, post_id.to_string())
        .await
    {
        Ok(_) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(GenericResponse {
                ok: true,
                msg: "success".to_string(),
            }),
        Err(err) => match err {
            Error::Duplicate => bad_request_response("invalid title".to_string()),
            Error::NotFound => bad_request_response("invalid id".to_string()),
            Error::Custom(_) => internal_server_error_response("db error".to_string()),
        },
    }
}
