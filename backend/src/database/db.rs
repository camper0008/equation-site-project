use crate::database::mongo_db::MongoDb;
use std::fmt;

pub enum DbError {
    ConnectionIssue,
    NotFound,
    Duplicate,
    Custom(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DbError::ConnectionIssue => "connection issue".to_string(),
                DbError::Duplicate => "duplicate".to_string(),
                DbError::NotFound => "not found".to_string(),
                DbError::Custom(s) => s.to_string(),
            }
        )
    }
}

impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type Db = MongoDb;