use crate::models::{
    DbEquation, DbUser, InsertableDbEquation, InsertableDbSession, InsertableDbUser,
    PreviewableEquation, SessionToken,
};
use async_trait::async_trait;
use futures::lock::Mutex;

pub type DbParam = Mutex<dyn Db + Send + Sync>;

#[derive(Debug)]
pub enum Error {
    Duplicate,
    NotFound,
    OpenSSL,
    Network,
    Custom(String),
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::OpenSSL
    }
}

#[cfg(feature = "mongo")]
impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::InvalidArgument { .. } => Error::NotFound,
            unmatched_err => Error::Custom(unmatched_err.to_string()),
        }
    }
}

#[async_trait]
pub trait Db {
    async fn add_user(&mut self, insertable_user: InsertableDbUser) -> Result<(), Error>;
    async fn user_from_name(&self, username: String) -> Result<DbUser, Error>;
    async fn add_equation(
        &mut self,
        insertable_equation: InsertableDbEquation,
    ) -> Result<(), Error>;
    async fn update_equation_from_id(
        &mut self,
        insertable_equation: InsertableDbEquation,
        post_id: String,
    ) -> Result<(), Error>;
    async fn equation_from_id(&self, id: String) -> Result<DbEquation, Error>;
    async fn equation_from_title(&self, title: String) -> Result<DbEquation, Error>;
    async fn all_titles(&self) -> Result<Vec<PreviewableEquation>, Error>;
    async fn add_session(&mut self, insertable_session: InsertableDbSession) -> Result<(), Error>;
    async fn session_user_from_token(&mut self, token: SessionToken) -> Result<DbUser, Error>;
    async fn delete_user_session(&mut self, token: SessionToken) -> Result<(), Error>;
}
