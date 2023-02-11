use crate::models::GenericResponse;
use actix_web::{
    cookie::Cookie,
    http::header::{ContentType, HeaderMap},
    HttpResponse,
};
use chrono::prelude::Utc;
use openssl::rand::rand_bytes;

pub fn utc_date_iso_string() -> String {
    Utc::now().to_rfc3339()
}

#[derive(Debug)]
pub enum GenRandomError {
    OpenSSLError,
}

fn gen_random_valid_chars() -> Result<[char; 64], GenRandomError> {
    const VALID_CHARACTERS: [char; 62] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    // because a u8 goes from 0-255
    const MAX_CHARACTERS: f64 = 255.0;

    let mut token_buffer = [0; 64];
    let res = rand_bytes(&mut token_buffer);
    if res.is_err() {
        return Err(GenRandomError::OpenSSLError);
    }

    const CHARACTERS_MAX_INDEX: f64 = 61.0;

    // this casts the char => u64 => f64, then converts it to a percentage with division and picks
    // from the VALID_CHARACTERS array based on that percentage.
    // this is because rand_bytes literally picks random bytes from 0-255, which sometimes include
    // control characters that are not allowed in headers, leading to an invalid header error
    Ok(token_buffer.map(|n| {
        VALID_CHARACTERS[(((u64::from(n) as f64) / MAX_CHARACTERS) * CHARACTERS_MAX_INDEX) as usize]
    }))
}

pub fn gen_64_char_random_valid_string() -> Result<String, GenRandomError> {
    let random_chars_result = gen_random_valid_chars();
    if random_chars_result.is_err() {
        return Err(random_chars_result.err().unwrap());
    };

    Ok(random_chars_result
        .ok()
        .unwrap()
        .into_iter()
        .collect::<String>())
}

pub fn gen_8_char_random_valid_string() -> Result<String, GenRandomError> {
    let random_chars_result = gen_random_valid_chars();
    if random_chars_result.is_err() {
        return Err(random_chars_result.err().unwrap());
    };

    Ok(random_chars_result
        .ok()
        .unwrap()
        .into_iter()
        .take(8)
        .collect::<String>())
}

pub fn internal_server_error_response(msg: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header(ContentType::json())
        .json(GenericResponse { ok: false, msg })
}

pub fn bad_request_response(msg: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .insert_header(ContentType::json())
        .json(GenericResponse { ok: false, msg })
}

pub enum CookieHeaderError {
    Malformed,
    NotIncluded,
}

pub fn cookie_from_header(headers: &HeaderMap) -> Result<Cookie, CookieHeaderError> {
    let cookie_header_option = headers.get("Cookie");
    if cookie_header_option.is_none() {
        return Err(CookieHeaderError::NotIncluded);
    }

    let cookie_header_stringify_result = cookie_header_option.unwrap().to_str();
    if cookie_header_stringify_result.is_err() {
        return Err(CookieHeaderError::Malformed);
    }

    let cookie_parse_result = Cookie::parse(cookie_header_stringify_result.unwrap());
    if cookie_parse_result.is_err() {
        return Err(CookieHeaderError::Malformed);
    };

    Ok(cookie_parse_result.unwrap())
}
