use crate::database::db::Db;
use crate::models::{GenericResponse, InsertableDbSession};
use crate::utils::{bad_request_response, gen_random_valid_string, internal_server_error_response};
use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    http::header::ContentType,
    post, web, HttpResponse, Responder,
};
use bcrypt::verify;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/users/login")]
pub async fn login(db: web::Data<Mutex<Db>>, req: web::Json<LoginRequest>) -> impl Responder {
    let result = (**db)
        .lock()
        .unwrap()
        .get_user_from_name(req.username.clone())
        .await;

    if result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = result.unwrap();
    if found.is_none() {
        return bad_request_response("invalid login".to_string());
    }
    let user = found.unwrap();
    let bcrypt_res = verify(req.password.clone(), &user.password);
    if bcrypt_res.is_err() {
        return internal_server_error_response("bcrypt error".to_string());
    };

    let matches = bcrypt_res.unwrap();
    if !matches {
        return bad_request_response("invalid login".to_string());
    };

    let random_string_result = gen_random_valid_string();
    if random_string_result.is_err() {
        return internal_server_error_response("openssl error".to_string());
    };

    let random_token_string = random_string_result.unwrap();
    let session = InsertableDbSession {
        user_id: user.id,
        token: random_token_string.to_string(),
    };

    let db_result = (**db).lock().unwrap().add_session(session).await;
    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .cookie(
            Cookie::build("SESSION_TOKEN", random_token_string)
                .http_only(true)
                .max_age(Duration::weeks(1))
                .same_site(SameSite::Lax) // TODO: set to strict + add secure attribute
                .path("/")
                .finish(),
        )
        .json(GenericResponse {
            ok: true,
            msg: "success".to_string(),
        })
}
