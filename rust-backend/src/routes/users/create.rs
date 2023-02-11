use crate::database::db::{Db, Error};
use crate::models::{GenericResponse, InsertableDbUser, Permission};
use crate::response_helper::{bad_request_response, internal_server_error_response};
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use futures::lock::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[post("/users/create")]
pub async fn create(db: web::Data<Mutex<Db>>, req: web::Json<Request>) -> impl Responder {
    let mut db = (**db).lock().await;
    let user_get_result = db.user_from_name(req.username.clone()).await;

    if user_get_result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = user_get_result.ok().unwrap();
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

    match db.add_user(user).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(GenericResponse {
                ok: true,
                msg: "success".to_string(),
            }),
        Err(err) => match err {
            Error::Duplicate => bad_request_response("invalid username".to_string()),
            _ => internal_server_error_response("db error".to_string()),
        },
    }
}
