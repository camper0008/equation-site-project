use crate::database::mongo_db::MongoDb;

pub enum DbError {
    Duplicate,
    NotFound,
    Custom(String),
}

pub type Db = MongoDb;
