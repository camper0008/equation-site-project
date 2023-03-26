use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
    char_generation::gen_8_char_random_valid_string,
    date_helper::utc_date_iso_string,
    models::{
        DbEquation, DbUser, InsertableDbEquation, InsertableDbSession, InsertableDbUser,
        PreviewableEquation, SessionToken,
    },
};

use super::db::{Db, Error};

pub struct Driver {
    pool: SqlitePool,
}

impl Driver {
    pub async fn new(db_url: String) -> Self {
        let pool = SqlitePool::connect(&db_url)
            .await
            .expect("unable to connect to db");

        Self { pool }
    }

    pub async fn title_is_taken(&self, id: &str, title: &str) -> Result<bool, Error> {
        match sqlx::query!("SELECT id FROM equations WHERE title=?", title)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(Some(other_id)) => Ok(other_id.id != id),
            Ok(None) => Ok(false),
            Err(_) => Err(Error::Network),
        }
    }
}

#[async_trait]
impl Db for Driver {
    async fn add_user(&mut self, insertable_user: InsertableDbUser) -> Result<(), Error> {
        let id = gen_8_char_random_valid_string()?;
        let date = utc_date_iso_string();
        sqlx::query!(
            "INSERT INTO users (id, username, password, permission, date_created) VALUES (?, ?, ?, ?, ?);",
            id,
            insertable_user.username,
            insertable_user.password,
            insertable_user.permission,
            date,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| Error::Network)?;

        Ok(())
    }

    async fn user_from_name(&self, username: String) -> Result<DbUser, Error> {
        let record = sqlx::query!("SELECT * FROM users WHERE username=?;", username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Network)?
            .ok_or(Error::NotFound)?;

        let permission = match record.permission.as_str() {
            "Root" => crate::models::Permission::Root,
            "Contributor" => crate::models::Permission::Contributor,
            "User" => crate::models::Permission::User,
            _ => unreachable!("user has invalid permission"),
        };

        Ok(DbUser {
            id: record.id,
            username: record.username,
            password: record.password,
            date_created: record.date_created,
            permission,
        })
    }

    async fn add_equation(
        &mut self,
        insertable_equation: InsertableDbEquation,
    ) -> Result<(), Error> {
        let dupe = sqlx::query!(
            "SELECT id FROM equations WHERE title=?",
            insertable_equation.title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| Error::Network)?;
        if dupe.is_some() {
            return Err(Error::Duplicate);
        }

        let id = gen_8_char_random_valid_string()?;
        sqlx::query!(
            "INSERT INTO equations (id, title, content, creator_id) VALUES (?, ?, ?, ?);",
            id,
            insertable_equation.title,
            insertable_equation.content,
            insertable_equation.creator_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| Error::Network)?;

        Ok(())
    }

    async fn update_equation_from_id(
        &mut self,
        insertable_equation: InsertableDbEquation,
        post_id: String,
    ) -> Result<(), Error> {
        if self
            .title_is_taken(&post_id, &insertable_equation.title)
            .await?
        {
            return Err(Error::Duplicate);
        };

        sqlx::query!(
            "UPDATE equations SET title=?, content=?, creator_id=? WHERE id=?;",
            insertable_equation.title,
            insertable_equation.content,
            insertable_equation.creator_id,
            post_id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| Error::Network)?;

        Ok(())
    }

    async fn equation_from_id(&self, id: String) -> Result<DbEquation, Error> {
        sqlx::query_as!(DbEquation, "SELECT * FROM equations WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Network)?
            .ok_or(Error::NotFound)
    }

    async fn equation_from_title(&self, title: String) -> Result<DbEquation, Error> {
        sqlx::query_as!(DbEquation, "SELECT * FROM equations WHERE title=?;", title)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Network)?
            .ok_or(Error::NotFound)
    }

    async fn all_titles(&self) -> Result<Vec<PreviewableEquation>, Error> {
        sqlx::query_as!(
            PreviewableEquation,
            "SELECT id, title, date_created FROM equations"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| Error::Network)
    }

    async fn add_session(&mut self, insertable_session: InsertableDbSession) -> Result<(), Error> {
        let date = utc_date_iso_string();
        sqlx::query!(
            "INSERT INTO sessions (token, user_id, date_created) VALUES (?, ?, ?);",
            insertable_session.token,
            insertable_session.user_id,
            date,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| Error::Network)?;

        Ok(())
    }

    async fn session_user_from_token(&mut self, token: SessionToken) -> Result<DbUser, Error> {
        let user_id = sqlx::query!("SELECT user_id FROM sessions WHERE token=?;", token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Network)?
            .ok_or(Error::NotFound)?
            .user_id;

        let record = sqlx::query!("SELECT * FROM users WHERE id=?;", user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Network)?
            .ok_or(Error::NotFound)?;

        let permission = match record.permission.as_str() {
            "Root" => crate::models::Permission::Root,
            "Contributor" => crate::models::Permission::Contributor,
            "User" => crate::models::Permission::User,
            _ => unreachable!("user has invalid permission"),
        };

        Ok(DbUser {
            id: record.id,
            username: record.username,
            password: record.password,
            date_created: record.date_created,
            permission,
        })
    }

    async fn delete_user_session(&mut self, token: SessionToken) -> Result<(), Error> {
        sqlx::query!("DELETE FROM sessions WHERE token=?;", token)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Network)?;

        Ok(())
    }
}
