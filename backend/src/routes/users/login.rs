use crate::database::db_trait::Db;
use crate::models::DbSession;
use actix_web::{cookie::Cookie, http::header::ContentType, post, web, HttpResponse, Responder};
use bcrypt::verify;
use openssl::{error::ErrorStack, rand::rand_bytes};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    ok: bool,
    msg: String,
}

fn get_random_valid_bytes() -> Result<[u8; 64], ErrorStack> {
    let mut token_buffer = [0; 64];
    rand_bytes(&mut token_buffer)?;

    // because a u8 goes from 0-255
    const MAX_CHARACTERS: f64 = 255.0;

    // because we want from the 62nd ascii character, since 61 is = and that might mess with token
    // header
    const CHARACTERS_STARTING_POINT: f64 = 62.0;

    // because we are picking from characters 62-90 in the ascii table
    const AMOUNT_OF_CHARACTERS: f64 = 28.0;

    // this casts the u8 => u64 => f64, then converts it to a percentage with division and
    // then just does a basic clamp function, then casts it back to a u8
    // this is because rand_bytes literally picks random bytes from 0-255, which sometimes include
    // control characters that are not allowed in headers, leading to an invalid header error
    Ok(token_buffer.map(|n| {
        ((((n as u64 as f64) / MAX_CHARACTERS) * AMOUNT_OF_CHARACTERS) + CHARACTERS_STARTING_POINT)
            as u64 as u8
    }))
}

fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(LoginResponse {
            ok: false,
            msg: msg,
        })
}

fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(LoginResponse {
            ok: false,
            msg: msg,
        })
}

#[post("/users/login")]
pub async fn login(db: web::Data<Mutex<Db>>, request: web::Json<LoginRequest>) -> impl Responder {
    let result = (**db)
        .lock()
        .unwrap()
        .get_user_from_name(request.username.clone())
        .await;

    if result.is_err() {
        return internal_server_error_response("db error".to_string());
    }

    let found = result.unwrap();
    if found.is_none() {
        return bad_request_response("invalid login".to_string());
    }
    let user = found.unwrap();
    let bcrypt_res = verify(request.password.clone(), &user.password);
    if bcrypt_res.is_err() {
        return internal_server_error_response("bcrypt error".to_string());
    };

    let matches = bcrypt_res.unwrap();
    if !matches {
        return bad_request_response("invalid login".to_string());
    };

    let random_token_result = get_random_valid_bytes();
    if random_token_result.is_err() {
        return internal_server_error_response("openssl error".to_string());
    };

    let random_token_string = String::from_utf8_lossy(random_string_result.unwrap());
    let session = DbSession {
        user_id: user.id,
        token: token.to_string(),
    };

    let db_result = (**db).lock().unwrap().add_session(session).await;
    if db_result.is_err() {
        return internal_server_error_response("db error".to_string());
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .cookie(Cookie::build("token", token).http_only(true).finish())
        .json(LoginResponse {
            ok: true,
            msg: "success".to_string(),
        })
}
