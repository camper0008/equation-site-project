use actix_web::{
    cookie::Cookie, http::header::ContentType, post, web, HttpResponse, Responder, Result,
};
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

#[post("/users/login")]
pub async fn login(request: web::Json<LoginRequest>) -> Result<impl Responder> {
    if request.username == "root" && request.password == "passwd" {
        let cookie = Cookie::build("token", "auth").http_only(true).finish();
        let res = LoginResponse {
            ok: true,
            msg: "success".into(),
        };
        Ok(HttpResponse::Ok()
            .insert_header(ContentType::json())
            .cookie(cookie)
            .json(res))
    } else {
        let res = LoginResponse {
            ok: false,
            msg: "invalid login".into(),
        };
        Ok(HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(res))
    }
}
