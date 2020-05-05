use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum ErrorRes {
    Basic { message: String },
}
impl fmt::Display for ErrorRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorRes::Basic { message } => write!(f, "{}", message),
        }
    }
}
impl Error for ErrorRes {}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginResponse {
    pub(crate) success: bool,
    pub(crate) token: String,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CustomResult<T> {
    Ok(T),
    Err(ErrorRes),
}

impl<T> From<CustomResult<T>> for Result<T, ErrorRes> {
    fn from(from: CustomResult<T>) -> Self {
        match from {
            CustomResult::Ok(x) => Ok(x),
            CustomResult::Err(x) => Err(x),
        }
    }
}
