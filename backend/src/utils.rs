use crate::models::GenericResponse;
use actix_web::{http::header::ContentType, HttpResponse};
use chrono::prelude::Utc;
use openssl::rand::rand_bytes;

pub fn utc_date_iso_string() -> String {
    Utc::now().to_rfc3339()
}

#[derive(Debug)]
pub enum GenRandomError {
    OpenSSLError,
    ConversionError,
}

fn gen_random_valid_bytes() -> Result<[u8; 64], GenRandomError> {
    let mut token_buffer = [0; 64];
    let res = rand_bytes(&mut token_buffer);
    if res.is_err() {
        return Err(GenRandomError::OpenSSLError);
    }

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

pub fn gen_random_valid_string() -> Result<String, GenRandomError> {
    let random_bytes_result = gen_random_valid_bytes();
    if random_bytes_result.is_err() {
        return Err(GenRandomError::OpenSSLError);
    };

    let random_bytes = random_bytes_result.unwrap().to_vec();
    let random_string_result = String::from_utf8(random_bytes);
    match random_string_result {
        Ok(random_string) => Ok(random_string),
        Err(_) => Err(GenRandomError::ConversionError),
    }
}

pub fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(GenericResponse {
            ok: false,
            msg: msg,
        })
}

pub fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(GenericResponse {
            ok: false,
            msg: msg,
        })
}
