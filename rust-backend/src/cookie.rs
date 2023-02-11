use actix_web::{cookie::Cookie, http::header::HeaderMap};

pub enum Error {
    Malformed,
    NotIncluded,
}

pub fn from_header(headers: &HeaderMap) -> Result<Cookie, Error> {
    let Some(cookie_header_option) = headers.get("Cookie") else {
        return Err(Error::NotIncluded);
    };

    let Ok(cookie_header_stringify_result) = cookie_header_option.to_str() else {
        return Err(Error::Malformed);
    };

    let Ok(parsed_cookie) = Cookie::parse(cookie_header_stringify_result) else {
        return Err(Error::Malformed);
    };

    Ok(parsed_cookie)
}
