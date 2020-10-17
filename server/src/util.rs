use crate::errors::ReturnError;
use futures::Future;
use warp::{Rejection, Reply};

pub(crate) fn cast_result<T>(data: Result<T, impl Into<ReturnError>>) -> Result<T, Rejection> {
    match data {
        Ok(x) => Ok(x),
        Err(x) => Err(warp::reject::custom::<ReturnError>(x.into())),
    }
}

pub trait CastRejection {
    type ToCast;
    fn half_cast(self) -> Result<Self::ToCast, ReturnError>;
    fn cast(self) -> Result<Self::ToCast, Rejection>;
}

impl<T, E: Into<ReturnError>> CastRejection for Result<T, E> {
    type ToCast = T;
    fn half_cast(self) -> Result<Self::ToCast, ReturnError> {
        match self {
            Ok(x) => Ok(x),
            Err(x) => Err(x.into()),
        }
    }
    fn cast(self) -> Result<Self::ToCast, Rejection> {
        cast_result(self)
    }
}

pub(crate) async fn convert_error<Func, Fut, Rep, State>(
    state: State,
    func: Func,
) -> Result<Rep, Rejection>
where
    Rep: Reply,
    Fut: Future<Output = Result<Rep, ReturnError>>,
    Func: Fn(State) -> Fut,
{
    Ok(func(state).await?)
}
