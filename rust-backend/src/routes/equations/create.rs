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

#[post("/equations/create")]
pub async fn create(
    db: web::Data<Mutex<Db>>,
    req: HttpRequest,
    body: web::Json<Request>,
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

    let user_get_result = db.session_user_from_token(cookie).await;
    let user = match user_get_result {
        Ok(user) => user,
        Err(db::Error::NotFound) => return bad_request_response("invalid cookie".to_string()),
        Err(_) => return internal_server_error_response("db error".to_string()),
    };

    let (Permission::Contributor | Permission::Root) = user.permission else {
        return bad_request_response("unauthorized".to_string());
    };

    let equation_get_result = db.equation_from_title(body.title.clone()).await;
    match equation_get_result {
        Err(db::Error::NotFound) => {}
        Ok(_) => return bad_request_response("invalid title".to_string()),
        Err(_) => return internal_server_error_response("db error".to_string()),
    }

    let equation = InsertableDbEquation {
        title: body.title.clone(),
        content: body.content.clone(),
        creator_id: user.id,
    };

    match db.add_equation(equation).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(GenericResponse {
                ok: true,
                msg: "success".to_string(),
            }),
        Err(db::Error::Duplicate) => bad_request_response("invalid title".to_string()),
        Err(_) => internal_server_error_response("db error".to_string()),
    }
}
