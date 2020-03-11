use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub(crate) enum ErrorRes {
    Basic { message: String },
}

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
