use argonautica::Error as HashError;
use card_game_shared::battle::BattleErrors;
use sqlx::{error::DatabaseError, Error as SqlError};
use std::io::Error as ioError;

#[derive(Debug)]
pub enum ReturnError {
    Io(ioError),
    GenericError(String),
    DatabaseError(Box<dyn DatabaseError + 'static>),
    NotFound,
    HashError(HashError),
    CustomError(String, warp::http::StatusCode),
    JsonError(serde_json::error::Error),
    BattleErrors(BattleErrors),
    LuaError(rlua::Error),
}

impl ReturnError {
    pub(crate) fn custom(message: impl ToString, error: warp::http::StatusCode) -> Self {
        Self::CustomError(message.to_string(), error)
    }
    pub fn map_not_found(self, run: impl Fn() -> ReturnError) -> ReturnError {
        match self {
            Self::NotFound => run(),
            x => x,
        }
    }
}

impl warp::reject::Reject for ReturnError {}

impl From<SqlError> for ReturnError {
    fn from(x: SqlError) -> Self {
        match x {
            SqlError::Io(a) => ReturnError::Io(a),
            SqlError::Database(x) => ReturnError::DatabaseError(x),
            SqlError::RowNotFound => ReturnError::NotFound,
            SqlError::ColumnNotFound(x) => {
                ReturnError::GenericError(format!("Collumn {:0} not found", x))
            }
            SqlError::Protocol(x) => ReturnError::GenericError(
                String::from("Something wend wrong with the database connection\nContenxt:\n") + &x,
            ),
            SqlError::PoolClosed => ReturnError::GenericError("The pool got closed".into()),
            SqlError::Decode(_) => ReturnError::GenericError("Couldn't decode something".into()),
            SqlError::ColumnIndexOutOfBounds { index, len } => ReturnError::GenericError(format!(
                "collomn {} is out of bounds. Len {}",
                index, len
            )),
            SqlError::Tls(_) => ReturnError::GenericError("Couldn't upgrade the tls".into()),
            _ => ReturnError::GenericError("Something wend wrong with the database".into()),
        }
    }
}
impl From<HashError> for ReturnError {
    fn from(error: HashError) -> Self {
        ReturnError::HashError(error)
    }
}

impl From<ReturnError> for warp::reject::Rejection {
    fn from(error: ReturnError) -> Self {
        warp::reject::custom(error)
    }
}

impl From<std::io::Error> for ReturnError {
    fn from(error: std::io::Error) -> Self {
        ReturnError::Io(error)
    }
}
impl From<serde_json::error::Error> for ReturnError {
    fn from(error: serde_json::error::Error) -> Self {
        ReturnError::JsonError(error)
    }
}

impl From<BattleErrors> for ReturnError {
    fn from(error: BattleErrors) -> Self {
        ReturnError::BattleErrors(error)
    }
}

impl From<rlua::Error> for ReturnError {
    fn from(error: rlua::Error) -> Self {
        ReturnError::LuaError(error)
    }
}
