use crate::database::db_trait::DbError;
use crate::models::{DbEquation, DbUser, Permission};
use mongodb::Client;

#[derive(Clone)]
pub struct MongoDb {
    client: Client,
    db_name: String,
}

impl MongoDb {
    pub async fn new(uri: String, db_name: String) -> Self {
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        Self {
            client: client,
            db_name: db_name,
        }
    }
    pub async fn add_user(&mut self, user: DbUser) -> Result<(), DbError> {
        let collection = self.client.database(&self.db_name).collection("users");
        let result = collection.insert_one(user, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn get_user_from_name(&self, name: String) -> Result<Option<DbUser>, DbError> {
        Ok(Some(DbUser {
            id: "root".to_string(),
            username: "root".to_string(),
            permission: Permission::Root,
            posts: vec![],
            date_created: "1970-01-01T00:00:00.000Z".to_string(), // ISO string
            //password: "passwd".to_string(),
            password: "$2y$12$/9ahkt3cP8aCoiXDQQiRleRfzuD6Xn6j5XtPZWfGpHGmsDDxLb/16".to_string(), // "passwd" encrypted with a cost of 12
        }))
    }
    pub async fn get_user_from_id(&self, id: String) -> Result<Option<DbUser>, DbError> {
        Ok(Some(DbUser {
            id: "".to_string(),
            username: "".to_string(),
            permission: Permission::Unauthenticated,
            posts: vec![],
            date_created: "".to_string(), // ISO string
            password: "".to_string(),
        }))
    }
    pub async fn add_equation(&mut self, equation: DbEquation) -> Result<(), DbError> {
        Ok(())
    }
    pub async fn get_equation_from_id(&self, id: String) -> Result<Option<DbEquation>, DbError> {
        Ok(Some(DbEquation {
            id: "".to_string(), // randomly generated
            title: "".to_string(),
            content: vec![],
            date_created: "".to_string(), // date created as ISO string
            creator: DbUser {
                id: "".to_string(),
                username: "".to_string(),
                permission: Permission::Unauthenticated,
                posts: vec![],
                date_created: "".to_string(), // ISO string
                password: "".to_string(),
            },
        }))
    }
}
