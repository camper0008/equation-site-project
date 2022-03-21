use crate::database::db::Db;
use crate::models::{InsertableDbUser, Permission};
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct CreateRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    ok: bool,
    msg: String,
}

fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(CreateResponse {
            ok: false,
            msg: msg,
        })
}

fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(CreateResponse {
            ok: false,
            msg: msg,
        })
}

#[post("/users/create")]
pub async fn create(db: web::Data<Mutex<Db>>, req: web::Json<CreateRequest>) -> impl Responder {
    let user_get_result = (**db)
        .lock()
        .unwrap()
        .get_user_from_name(req.username.clone())
        .await;

    if user_get_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = user_get_result.unwrap();
    if found.is_some() {
        return bad_request_response("invalid username".to_string());
    };

    let bcrypt_res = hash(req.password.clone(), DEFAULT_COST);
    if bcrypt_res.is_err() {
        return internal_server_error_response("bcrypt error".to_string());
    };

    let hashed = bcrypt_res.unwrap();

    let user = InsertableDbUser {
        username: req.username.clone(),
        permission: Permission::User,
        password: hashed,
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(CreateResponse {
            ok: true,
            msg: "success".to_string(),
        })
}
