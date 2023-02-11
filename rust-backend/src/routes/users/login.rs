use crate::char_generation::gen_64_char_random_valid_string;
use crate::database::db::{Db, Error};
use crate::models::{GenericResponse, InsertableDbSession};
use crate::response_helper::{bad_request_response, internal_server_error_response};
use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    http::header::ContentType,
    post, web, HttpResponse, Responder,
};
use bcrypt::verify;
use futures::lock::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[post("/users/login")]
pub async fn login(db: web::Data<Mutex<Db>>, req: web::Json<Request>) -> impl Responder {
    let mut db = (**db).lock().await;
    let user = match db.user_from_name(req.username.clone()).await {
        Ok(user) => user,
        Err(Error::NotFound) => {
            return bad_request_response("invalid login".to_string());
        }
        Err(_) => {
            return internal_server_error_response("db error".to_string());
        }
    };

    match verify(req.password.clone(), &user.password) {
        Ok(true) => {}
        Ok(false) => {
            return bad_request_response("invalid login".to_string());
        }
        Err(_) => {
            return internal_server_error_response("bcrypt error".to_string());
        }
    };

    let Ok(random_token_string) = gen_64_char_random_valid_string() else {
        return internal_server_error_response("openssl error".to_string());
    };

    let session = InsertableDbSession {
        user_id: user.id,
        token: random_token_string.clone(),
    };

    if let Err(_) = db.add_session(session).await {
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
