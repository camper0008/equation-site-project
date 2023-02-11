use crate::cookie;
use crate::database::db;
use crate::database::db::Db;
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
    let cookie = match cookie::from_header(req.headers()) {
        Ok(cookie) => cookie.value().to_string(),
        Err(err) => {
            return bad_request_response(match err {
                cookie::Error::Malformed => "malformed cookie header".to_string(),
                cookie::Error::NotIncluded => "cookie header not included".to_string(),
            });
        }
    };

    let mut db = (**db).lock().await;

    let user = match db.session_user_from_token(cookie).await {
        Ok(user) => user,
        Err(db::Error::NotFound) => return bad_request_response("invalid cookie".to_string()),
        Err(_) => return internal_server_error_response("db error".to_string()),
    };

    let (Permission::Contributor | Permission::Root) = user.permission else {
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
        Err(db::Error::Duplicate) => bad_request_response("invalid title".to_string()),
        Err(db::Error::NotFound) => bad_request_response("invalid id".to_string()),
        Err(db::Error::OpenSSL) => unreachable!("should never return openssl error"),
        Err(db::Error::Custom(_)) => internal_server_error_response("db error".to_string()),
    }
}
