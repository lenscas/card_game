use argonautica::Error as HashError;
use sqlx::{error::DatabaseError, Error as SqlError};
use std::io::Error as ioError;

#[derive(Debug)]
pub enum ReturnErrors {
    Io(ioError),
    GenericError(String),
    DatabaseError(Box<dyn DatabaseError + 'static + Send + Sync>),
    NotFound,
    HashError(HashError),
}
impl warp::reject::Reject for ReturnErrors {}

impl From<SqlError> for ReturnErrors {
    fn from(x: SqlError) -> Self {
        match x {
            SqlError::Io(a) => ReturnErrors::Io(a),
            SqlError::UrlParse(_) => ReturnErrors::GenericError("Couldn't parse db string".into()),
            SqlError::Database(x) => ReturnErrors::DatabaseError(x),
            SqlError::NotFound => ReturnErrors::NotFound,
            SqlError::FoundMoreThanOne => ReturnErrors::GenericError(
                "More than one result got returned while only one was expected".into(),
            ),
            SqlError::ColumnNotFound(x) => {
                ReturnErrors::GenericError(format!("Collumn {:0} not found", x))
            }
            SqlError::Protocol(x) => ReturnErrors::GenericError(format!(
                "Something wend wrong with the database connection\nContenxt:\n{:0}",
                x
            )),
            SqlError::PoolTimedOut(_) => ReturnErrors::GenericError("The pool timed out".into()),
            SqlError::PoolClosed => ReturnErrors::GenericError("The pool got closed".into()),
            SqlError::TlsUpgrade(_) => {
                ReturnErrors::GenericError("Couldn't upgrade the tls".into())
            }
            SqlError::Decode(_) => ReturnErrors::GenericError("Couldn't decode something".into()),
            SqlError::__Nonexhaustive => {
                ReturnErrors::GenericError("Something wend wrong with the database".into())
            }
        }
    }
}
impl From<HashError> for ReturnErrors {
    fn from(error: HashError) -> Self {
        ReturnErrors::HashError(error)
    }
}
impl From<ReturnErrors> for warp::reject::Rejection {
    fn from(error: ReturnErrors) -> Self {
        warp::reject::custom(error)
    }
}
