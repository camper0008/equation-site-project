use crate::database::mongo_db::MongoDb;

pub enum DbError {
    Duplicate,
    Custom(String),
}

pub type Db = MongoDb;
