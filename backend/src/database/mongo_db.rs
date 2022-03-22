use crate::database::db::DbError;
use crate::models::{
    DbEquation, DbSession, DbUser, InsertableDbSession, InsertableDbUser, SessionToken,
};
use crate::utils::{gen_random_valid_string, utc_date_iso_string};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use std::time::Duration;

async fn create_session_expire_index(client: &Client, db_name: &str) {
    let options = IndexOptions::builder()
        .expire_after(Duration::from_secs(60 * 60 * 24 * 7))
        .build();
    let model = IndexModel::builder()
        .keys(doc! { "date_created": 1 })
        .options(options)
        .build();
    client
        .database(db_name)
        .collection::<DbSession>("sessions")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[derive(Clone)]
pub struct MongoDb {
    client: Client,
    db_name: String,
}

impl MongoDb {
    pub async fn new(uri: String, db_name: String) -> Self {
        let client = Client::with_uri_str(uri).await.expect("failed to connect");

        create_session_expire_index(&client, &db_name).await;

        Self {
            client: client,
            db_name: db_name,
        }
    }
    pub async fn add_user(&mut self, insertable_user: InsertableDbUser) -> Result<(), DbError> {
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");
        let duplicate_user_result = match collection
            .find_one(doc! { "username": &insertable_user.username.clone() }, None)
            .await
        {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        };

        let duplicate_user = duplicate_user_result?;
        if duplicate_user.is_some() {
            return Err(DbError::Duplicate);
        };

        let random_id_result = match gen_random_valid_string() {
            Ok(random_id) => Ok(random_id),
            Err(_) => Err(DbError::Custom("openssl error".to_string())),
        };
        let random_id = random_id_result?;

        let user = DbUser {
            id: random_id,
            username: insertable_user.username,
            password: insertable_user.password,
            permission: insertable_user.permission,
            posts: vec![],
            date_created: utc_date_iso_string(),
        };

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
    pub async fn add_session(
        &mut self,
        insertable_session: InsertableDbSession,
    ) -> Result<(), DbError> {
        let collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");

        let session = DbSession {
            token: insertable_session.token,
            user_id: insertable_session.user_id,
            date_created: utc_date_iso_string(),
        };

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
            .find_one(doc! { "id": session.user_id }, None)
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
        match collection
            .find_one_and_delete(doc! { "token": token }, None)
            .await
        {
            Ok(Some(session)) => Ok(Some(session)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }
}
