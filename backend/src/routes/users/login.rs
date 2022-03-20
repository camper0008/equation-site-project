use crate::database::db_trait::Db;
use actix_web::{cookie::Cookie, http::header::ContentType, post, web, HttpResponse, Responder};
use bcrypt::verify;
use openssl::{error::ErrorStack, rand::rand_bytes};
use serde::{Deserialize, Serialize};

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

fn get_random_valid_string() -> Result<[u8; 64], ErrorStack> {
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

#[post("/users/login")]
pub async fn login(db: web::Data<Db>, request: web::Json<LoginRequest>) -> impl Responder {
    let result = db.get_user_from_name(request.username.clone()).await;

    // TODO: find a way to clean this match tree up

    match result {
        Ok(is_found) => match is_found {
            Some(user) => {
                let bcrypt_res = verify(request.password.clone(), &user.password);
                match bcrypt_res {
                    Ok(valid) => {
                        if valid {
                            let result = get_random_valid_string();
                            match result {
                                Ok(random_token) => HttpResponse::Ok()
                                    .insert_header(ContentType::json())
                                    .cookie(
                                        Cookie::build(
                                            "token",
                                            String::from_utf8_lossy(&random_token),
                                        )
                                        .http_only(true)
                                        .finish(),
                                    )
                                    .json(LoginResponse {
                                        ok: true,
                                        msg: "success".into(),
                                    }),
                                Err(_) => HttpResponse::InternalServerError()
                                    .insert_header(ContentType::json())
                                    .json(LoginResponse {
                                        ok: false,
                                        msg: "openssl error".to_string(),
                                    }),
                            }
                        } else {
                            HttpResponse::BadRequest()
                                .insert_header(ContentType::json())
                                .json(LoginResponse {
                                    ok: false,
                                    msg: "invalid login".to_string(),
                                })
                        }
                    }
                    Err(err) => HttpResponse::InternalServerError()
                        .insert_header(ContentType::json())
                        .json(LoginResponse {
                            ok: false,
                            msg: format!("bcrypt error: {}", err),
                        }),
                }
            }
            None => HttpResponse::BadRequest()
                .insert_header(ContentType::json())
                .json(LoginResponse {
                    ok: false,
                    msg: "invalid login".to_string(),
                }),
        },
        Err(err) => HttpResponse::InternalServerError()
            .insert_header(ContentType::json())
            .json(LoginResponse {
                ok: false,
                msg: err.to_string(),
            }),
    }
}
