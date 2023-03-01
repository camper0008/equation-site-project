use crate::char_generation::gen_8_char_random_valid_string;
use crate::database::db::Error;
use crate::date_helper::utc_date_iso_string;
use crate::models::{
    DbEquation, DbSession, DbUser, InsertableDbEquation, InsertableDbSession, InsertableDbUser,
    PreviewableEquation, SessionToken,
};
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::doc, options::FindOptions, options::IndexOptions, Client, Collection, IndexModel,
};
use std::convert::From;
use std::time::Duration;

use super::db::Db;

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

        Self { client, db_name }
    }
}

#[async_trait]
impl Db for MongoDb {
    async fn add_user(&mut self, insertable_user: InsertableDbUser) -> Result<(), Error> {
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");

        match self.user_from_name(insertable_user.username.clone()).await {
            Err(Error::NotFound) => Ok(()),
            Ok(_) => Err(Error::Duplicate),
            err => err.map(|_| ()),
        }?;

        let random_id = gen_8_char_random_valid_string()?;

        let user = DbUser {
            id: random_id,
            username: insertable_user.username,
            password: insertable_user.password,
            permission: insertable_user.permission,
            date_created: utc_date_iso_string(),
        };

        collection.insert_one(user, None).await?;

        Ok(())
    }

    async fn user_from_name(&self, username: String) -> Result<DbUser, Error> {
        let collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");

        match collection
            .find_one(doc! { "username": username }, None)
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::Custom(err.to_string())),
        }
    }

    async fn add_equation(
        &mut self,
        insertable_equation: InsertableDbEquation,
    ) -> Result<(), Error> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");

        match self
            .equation_from_title(insertable_equation.title.clone())
            .await
        {
            Err(Error::NotFound) => Ok(()),
            Ok(_) => Err(Error::Duplicate),
            err => err.map(|_| ()),
        }?;

        let random_id = gen_8_char_random_valid_string()?;

        let equation = DbEquation {
            id: random_id,
            title: insertable_equation.title,
            creator_id: insertable_equation.creator_id,
            content: insertable_equation.content,
            date_created: utc_date_iso_string(),
        };

        collection.insert_one(equation, None).await?;

        Ok(())
    }

    async fn update_equation_from_id(
        &mut self,
        insertable_equation: InsertableDbEquation,
        post_id: String,
    ) -> Result<(), Error> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");

        match self
            .equation_from_title(insertable_equation.title.clone())
            .await
        {
            Ok(DbEquation { id, .. }) if id == post_id => Ok(()),
            Ok(_) => Err(Error::Duplicate),
            err => err.map(|_| ()),
        }?;

        self.equation_from_id(post_id.clone()).await?;

        collection
            .update_one(
                doc! {
                    "id": post_id
                },
                doc! {
                    "$set": {
                        "title": insertable_equation.title,
                        "content": insertable_equation.content,
                    },
                },
                None,
            )
            .await?;

        Ok(())
    }

    async fn equation_from_id(&self, id: String) -> Result<DbEquation, Error> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");

        match collection.find_one(doc! { "id": id }, None).await {
            Ok(Some(equation)) => Ok(equation),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn equation_from_title(&self, title: String) -> Result<DbEquation, Error> {
        let collection: Collection<DbEquation> =
            self.client.database(&self.db_name).collection("equations");
        match collection.find_one(doc! { "title": title }, None).await {
            Ok(Some(equation)) => Ok(equation),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn all_titles(&self) -> Result<Vec<PreviewableEquation>, Error> {
        self.client
            .database(&self.db_name)
            .collection("equations")
            .find(
                doc! {},
                FindOptions::builder()
                    .projection(doc! {"id": 1u32, "title": 1u32, "date_created": 1u32})
                    .build(),
            )
            .await
            .map_err(|e| Error::Custom(format!("title recieving error, {e}")))?
            .try_collect()
            .await
            .map_err(|_| Error::Custom("title collection error".to_string()))
    }

    async fn add_session(&mut self, insertable_session: InsertableDbSession) -> Result<(), Error> {
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
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn session_user_from_token(&mut self, token: SessionToken) -> Result<DbUser, Error> {
        let session_collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");
        let session = match session_collection
            .find_one(doc! { "token": token }, None)
            .await
        {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::Custom(err.to_string())),
        }?;

        let user_collection: Collection<DbUser> =
            self.client.database(&self.db_name).collection("users");

        match user_collection
            .find_one(doc! { "id": session.user_id }, None)
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn delete_user_session(&mut self, token: SessionToken) -> Result<DbSession, Error> {
        let collection: Collection<DbSession> =
            self.client.database(&self.db_name).collection("sessions");
        match collection
            .find_one_and_delete(doc! { "token": token }, None)
            .await
        {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::from(err)),
        }
    }
}
