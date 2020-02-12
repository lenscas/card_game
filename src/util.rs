use crate::errors::ReturnErrors;
use warp::Rejection;

pub fn cast_result<T>(data: Result<T, impl Into<ReturnErrors>>) -> Result<T, Rejection> {
    match data {
        Ok(x) => Ok(x),
        Err(x) => Err(warp::reject::custom::<ReturnErrors>(x.into())),
    }
}

pub trait CastRejection {
    type ToCast;
    fn half_cast(self) -> Result<Self::ToCast, ReturnErrors>;
    fn cast(self) -> Result<Self::ToCast, Rejection>;
}

impl<T, E: Into<ReturnErrors>> CastRejection for Result<T, E> {
    type ToCast = T;
    fn half_cast(self) -> Result<Self::ToCast, ReturnErrors> {
        match self {
            Ok(x) => Ok(x),
            Err(x) => Err(x.into()),
        }
    }
    fn cast(self) -> Result<Self::ToCast, Rejection> {
        cast_result(self)
    }
}
