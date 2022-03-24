use crate::database::db::DbError;
use crate::models::{
    DbEquation, DbSession, DbUser, EquationContent, InsertableDbEquation, InsertableDbSession,
    InsertableDbUser, SessionToken,
};
use crate::utils::{gen_8_char_random_valid_string, utc_date_iso_string};
use futures::stream::StreamExt;
use mongodb::{bson::doc, bson::Bson, options::IndexOptions, Client, Collection, IndexModel};
use std::convert::From;
use std::future::ready;
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

async fn create_unique_text_title_index(client: &Client, db_name: &str) {
    let options = IndexOptions::builder().unique(true).build();
    let id_model = IndexModel::builder()
        .keys(doc! { "id": 1 })
        .options(options.clone())
        .build();
    client
        .database(db_name)
        .collection::<DbEquation>("equations")
        .create_index(id_model, None)
        .await
        .expect("creating an index should succeed");

    let title_model = IndexModel::builder()
        .keys(doc! { "title": "text" })
        .options(options)
        .build();

    client
        .database(db_name)
        .collection::<DbEquation>("equations")
        .create_index(title_model, None)
        .await
        .expect("creating an index should succeed");
}

async fn create_unique_username_index(client: &Client, db_name: &str) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "id": 1, "username": 1 })
        .options(options)
        .build();
    client
        .database(db_name)
        .collection::<DbUser>("users")
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
        create_unique_text_title_index(&client, &db_name).await;
        create_unique_username_index(&client, &db_name).await;

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

        let random_id_result = match gen_8_char_random_valid_string() {
            Ok(random_id) => Ok(random_id),
            Err(_) => Err(DbError::Custom("openssl error".to_string())),
        };
        let random_id = random_id_result?;

        let user = DbUser {
            id: random_id,
            username: insertable_user.username,
            password: insertable_user.password,
            permission: insertable_user.permission,
            date_created: utc_date_iso_string(),
        };

        let result = collection.insert_one(user, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }

    pub async fn user_from_name(&self, username: String) -> Result<Option<DbUser>, DbError> {
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

    pub async fn add_equation(
        &mut self,
        insertable_equation: InsertableDbEquation,
    ) -> Result<(), DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        let duplicate_equation_result = match collection
            .find_one(doc! { "title": &insertable_equation.title.clone() }, None)
            .await
        {
            Ok(Some(equation)) => Ok(Some(equation)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        };

        let duplicate_equation = duplicate_equation_result?;
        if duplicate_equation.is_some() {
            return Err(DbError::Duplicate);
        };

        let random_id_result = match gen_8_char_random_valid_string() {
            Ok(random_id) => Ok(random_id),
            Err(_) => Err(DbError::Custom("openssl error".to_string())),
        };
        let random_id = random_id_result?;

        let equation = DbEquation {
            id: random_id,
            title: insertable_equation.title,
            creator_id: insertable_equation.creator_id,
            content: insertable_equation.content,
            date_created: utc_date_iso_string(),
        };

        let result = collection.insert_one(equation, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }

    pub async fn update_equation_from_id(
        &mut self,
        insertable_equation: InsertableDbEquation,
        post_id: String,
    ) -> Result<(), DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        let duplicate_equation_result = match collection
            .find_one(doc! { "title": &insertable_equation.title.clone() }, None)
            .await
        {
            Ok(Some(equation)) => Ok(Some(equation)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        };

        let found_equation_option = duplicate_equation_result?;
        match found_equation_option {
            Some(ref equation) => {
                if equation.id != post_id {
                    return Err(DbError::Duplicate);
                }
            }
            None => return Err(DbError::NotFound),
        };

        let result = collection
            .update_one(
                doc! {
                    "id": post_id
                },
                doc! {
                    "title": insertable_equation.title,
                    "content": insertable_equation.content,
                },
                None,
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }

    pub async fn equation_from_id(&self, id: String) -> Result<Option<DbEquation>, DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        match collection.find_one(doc! { "id": id }, None).await {
            Ok(Some(equation)) => Ok(Some(equation)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }

    pub async fn equation_from_title(&self, title: String) -> Result<Option<DbEquation>, DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        match collection.find_one(doc! { "title": title }, None).await {
            Ok(Some(equation)) => Ok(Some(equation)),
            Ok(None) => Ok(None),
            Err(err) => Err(DbError::Custom(err.to_string())),
        }
    }

    pub async fn equation_search(&self, title: String) -> Result<Vec<DbEquation>, DbError> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");

        let result = collection
            .find(doc! { "$text": {"$search": title} }, None)
            .await;

        if result.is_err() {
            return Err(DbError::Custom(result.err().unwrap().to_string()));
        }

        let cursor = result.ok().unwrap();

        let equations: Vec<DbEquation> = cursor
            .enumerate()
            .filter(|(i, _)| ready(*i < 100))
            .filter_map(|(_, r)| ready(r.ok()))
            .collect::<Vec<DbEquation>>()
            .await;

        Ok(equations)
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

    pub async fn session_user_from_token(
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
            return Err(session_result.err().unwrap());
        };

        let session_or_none = session_result.ok().unwrap();
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

impl From<EquationContent> for Bson {
    fn from(ec: EquationContent) -> Self {
        Bson::Document(doc! {
            "value": Bson::String(ec.value),
                "content_type" : Bson::String(ec.content_type.to_string()),
        })
    }
}
