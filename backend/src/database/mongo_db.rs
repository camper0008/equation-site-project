use crate::database::db_trait::DbError;
use crate::models::{DbEquation, DbSession, DbUser, SessionToken};
use mongodb::{bson::doc, Client, Collection};

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
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");
        let result = collection.insert_one(user, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn get_user_from_name(&self, username: String) -> Result<Option<DbUser>, DbError> {
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");
        match collection
            .find_one(doc! { "username": username }, None)
            .await
        {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn get_user_from_id(&self, id: String) -> Result<Option<DbUser>, DbError> {
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");
        match collection.find_one(doc! { "id": id }, None).await {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn add_equation(&mut self, equation: DbEquation) -> Result<(), DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        let result = collection.insert_one(equation, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn get_equation_from_id(&self, id: String) -> Result<Option<DbEquation>, DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        match collection.find_one(doc! { "id": id }, None).await {
            Ok(Some(equation)) => Ok(Some(equation)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn add_session(&mut self, session: DbSession) -> Result<(), DbError> {
        let collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");
        let result = collection.insert_one(session, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
    pub async fn get_session_user_from_token(
        &mut self,
        token: SessionToken,
    ) -> Result<Option<DbUser>, DbError> {
        let session_collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");
        let session_result = match session_collection
            .find_one(doc! { "token": token }, None)
            .await
        {
            Ok(Some(session)) => Ok(Some(session)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        };

        if session_result.is_err() {
            return match session_result {
                Err(err) => Err(DbError::Custom(err.to_string())),
                _ => Ok(None),
            };
        };

        let session_or_none = session_result.unwrap();
        if session_or_none.is_none() {
            return Ok(None);
        };

        let session = session_or_none.unwrap();
        let user_collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");
        let user_result = match user_collection
            .find_one(doc! { "user_id": session.user_id }, None)
            .await
        {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        };

        user_result
    }
    pub async fn delete_user_session(
        &mut self,
        token: SessionToken,
    ) -> Result<Option<DbSession>, DbError> {
        let collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");
        match collection.find_one(doc! { "token": token }, None).await {
            Ok(Some(session)) => Ok(Some(session)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
}
