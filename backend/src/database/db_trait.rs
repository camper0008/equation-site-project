//use crate::models::{DbEquation, DbUser};
use crate::database::mongo_db::MongoDb;

pub enum DbError {
    ConnectionIssue,
    Duplicate,
    Custom(String),
}

impl DbError {
    pub fn to_string(&self) -> String {
        match self {
            DbError::ConnectionIssue => "connection issue".to_string(),
            DbError::Duplicate => "duplicate".to_string(),
            DbError::Custom(s) => s.to_string(),
        }
    }
}

pub type Db = MongoDb;

// traits are not ready to be async yet
//pub trait Db {
//    async fn add_user(&mut self, user: DbUser) -> dyn Future<Output = Result<(), DbError>>;
//    async fn get_user_from_id(
//        &self,
//        id: String,
//    ) -> dyn Future<Output = Result<Option<DbUser>, DbError>>;
//    async fn add_equation(
//        &mut self,
//        equation: DbEquation,
//    ) -> dyn Future<Output = Result<(), DbError>>;
//    async fn get_equation_from_id(
//        &self,
//        id: String,
//    ) -> dyn Future<Output = Result<Option<DbEquation>, DbError>>;
//}
