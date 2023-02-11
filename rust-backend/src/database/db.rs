use crate::database::mongo_db::MongoDb;

#[derive(Debug)]
pub enum Error {
    Duplicate,
    NotFound,
    OpenSSL,
    Custom(String),
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::OpenSSL
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::InvalidArgument { .. } => Error::NotFound,
            unmatched_err => Error::Custom(unmatched_err.to_string()),
        }
    }
}

pub type Db = MongoDb;
