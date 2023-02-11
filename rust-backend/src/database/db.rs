use crate::database::mongo_db::MongoDb;

#[derive(Debug)]
pub enum Error {
    Duplicate,
    NotFound,
    Custom(String),
}

pub type Db = MongoDb;
