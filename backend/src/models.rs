use serde::{Deserialize, Serialize};

pub type SessionToken = String;

#[derive(Serialize)]
pub struct GenericResponse {
    pub msg: String,
    pub ok: bool,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub permission: Permission,
    pub date_created: String, // ISO string
}

#[derive(Serialize, Deserialize)]
pub struct DbUser {
    pub id: String,
    pub username: String,
    pub permission: Permission,
    pub date_created: String, // ISO string
    pub password: String,
}

pub struct InsertableDbUser {
    pub username: String,
    pub password: String,
    pub permission: Permission,
}

#[derive(Serialize, Deserialize)]
pub struct DbEquation {
    pub id: String, // randomly generated
    pub title: String,
    pub content: String,
    pub date_created: String, // date created as ISO string
    pub creator_id: String,
}

pub struct InsertableDbEquation {
    pub title: String,
    pub content: String,
    pub creator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbSession {
    pub token: SessionToken,
    pub user_id: String,
    pub date_created: String, // ISO string
}

pub struct InsertableDbSession {
    pub token: SessionToken,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Permission {
    User,
    Contributor,
    Root,
}

pub type Equation = DbEquation;

// a shortened version of Equation to only provide necessary data to preview.
#[derive(Serialize, Deserialize)]
pub struct PreviewableEquation {
    pub id: String, // same as the full version of the equation
    pub title: String,
    pub date_created: String, // date created as ISO string
}
