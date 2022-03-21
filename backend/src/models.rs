use serde::{Deserialize, Serialize};

pub type SessionToken = String;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub permission: Permission,
    pub posts: Vec<String>,   // post ids
    pub date_created: String, // ISO string
}

#[derive(Serialize, Deserialize)]
pub struct DbUser {
    pub id: String,
    pub username: String,
    pub permission: Permission,
    pub posts: Vec<String>,   // post ids
    pub date_created: String, // ISO string
    pub password: String,
}

#[derive(Serialize)]
pub struct DbEquation {
    pub id: String, // randomly generated
    pub title: String,
    pub content: Vec<EquationContent>,
    pub date_created: String, // date created as ISO string
    pub creator_id: String,
}

#[derive(Serialize)]
pub struct DbSession {
    pub token: SessionToken,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    User,
    Contributor,
    Root,
}

#[derive(Serialize)]
pub struct Equation {
    id: String, // randomly generated
    title: String,
    content: Vec<EquationContent>,
    date_created: String, // date created as ISO string
    creator_id: String,
}

// a shortened version of Equation to only provide necessary data to preview.
#[derive(Serialize)]
pub struct PreviewableEquation {
    id: String, // same as the full version of the equation
    title: String,
    date_created: String, // date created as ISO string
}

#[derive(Serialize)]
pub struct EquationContent {
    content_type: EquationContentType,
    value: String,
}

#[derive(Serialize)]
pub enum EquationContentType {
    Text,
    Image,
    Math,
}
