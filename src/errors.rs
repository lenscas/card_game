use argonautica::Error as HashError;
use card_game_shared::battle::BattleErrors;
use sqlx::{error::DatabaseError, Error as SqlError};
use std::io::Error as ioError;

#[derive(Debug)]
pub(crate) enum ReturnErrors {
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

impl ReturnErrors {
    pub fn map_not_found(self, run: impl Fn() -> ReturnErrors) -> ReturnErrors {
        match self {
            Self::NotFound => run(),
            x => x,
        }
    }
}

impl warp::reject::Reject for ReturnErrors {}

impl From<SqlError> for ReturnErrors {
    fn from(x: SqlError) -> Self {
        match x {
            SqlError::Io(a) => ReturnErrors::Io(a),
            SqlError::UrlParse(_) => ReturnErrors::GenericError("Couldn't parse db string".into()),
            SqlError::Database(x) => ReturnErrors::DatabaseError(x),
            SqlError::RowNotFound => ReturnErrors::NotFound,
            SqlError::ColumnNotFound(x) => {
                ReturnErrors::GenericError(format!("Collumn {:0} not found", x))
            }
            SqlError::Protocol(x) => ReturnErrors::GenericError(format!(
                "Something wend wrong with the database connection\nContenxt:\n{:0}",
                x
            )),
            SqlError::PoolTimedOut(_) => ReturnErrors::GenericError("The pool timed out".into()),
            SqlError::PoolClosed => ReturnErrors::GenericError("The pool got closed".into()),
            SqlError::Decode(_) => ReturnErrors::GenericError("Couldn't decode something".into()),
            SqlError::ColumnIndexOutOfBounds { index, len } => ReturnErrors::GenericError(format!(
                "collomn {} is out of bounds. Len {}",
                index, len
            )),
            SqlError::Tls(_) => ReturnErrors::GenericError("Couldn't upgrade the tls".into()),
            _ => ReturnErrors::GenericError("Something wend wrong with the database".into()),
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

impl From<std::io::Error> for ReturnErrors {
    fn from(error: std::io::Error) -> Self {
        ReturnErrors::Io(error)
    }
}
impl From<serde_json::error::Error> for ReturnErrors {
    fn from(error: serde_json::error::Error) -> Self {
        ReturnErrors::JsonError(error)
    }
}

impl From<BattleErrors> for ReturnErrors {
    fn from(error: BattleErrors) -> Self {
        ReturnErrors::BattleErrors(error)
    }
}

impl From<rlua::Error> for ReturnErrors {
    fn from(error: rlua::Error) -> Self {
        ReturnErrors::LuaError(error)
    }
}
