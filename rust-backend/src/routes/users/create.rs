use crate::database::db::{DbParam, Error};
use crate::models::{GenericResponse, InsertableDbUser, Permission};
use crate::response_helper::{bad_request_response, internal_server_error_response};
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[post("/users/create")]
pub async fn create(db: web::Data<DbParam>, req: web::Json<Request>) -> impl Responder {
    let mut db = (**db).lock().await;
    let user_get_result = db.user_from_name(req.username.clone()).await;
    match user_get_result {
        Err(Error::NotFound) => {}
        Ok(_) => return bad_request_response("invalid username"),
        Err(_) => return internal_server_error_response("db error"),
    }

    let Ok(hashed_password) = hash(req.password.clone(), DEFAULT_COST) else {
        return internal_server_error_response("bcrypt error");
    };

    let user = InsertableDbUser {
        username: req.username.clone(),
        permission: Permission::User,
        password: hashed_password,
    };

    match db.add_user(user).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(GenericResponse {
                ok: true,
                msg: "success",
            }),
        Err(err) => match err {
            Error::Duplicate => bad_request_response("invalid username"),
            _ => internal_server_error_response("db error"),
        },
    }
}
