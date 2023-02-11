use actix_web::{cookie::Cookie, http::header::HeaderMap};

pub enum CookieHeaderError {
    Malformed,
    NotIncluded,
}

pub fn cookie_from_header(headers: &HeaderMap) -> Result<Cookie, CookieHeaderError> {
    let Some(cookie_header_option) = headers.get("Cookie") else {
        return Err(CookieHeaderError::NotIncluded);
    };

    let Ok(cookie_header_stringify_result) = cookie_header_option.to_str() else {
        return Err(CookieHeaderError::Malformed);
    };

    let Ok(parsed_cookie) = Cookie::parse(cookie_header_stringify_result) else {
        return Err(CookieHeaderError::Malformed);
    };

    Ok(parsed_cookie)
}
