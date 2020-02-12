use crate::users::force_logged_in;
use crate::util::CastRejection;
use dotenv::{dotenv, var};
use errors::ReturnErrors;
use serde_derive::Serialize;
use sqlx::{query, PgPool};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

mod errors;
mod users;
mod util;

async fn handle_from_db(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let mut con = pool.acquire().await.unwrap();
    query!("SELECT username FROM users WHERE id = $1", id)
        .fetch_one(&mut con)
        .await
        .map(|v| v.username)
        .cast()
}
#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set.");
    println!("Hello, world!");

    let pool = PgPool::new(&db_url)
        .await
        .expect("Couldn't connect to database");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let pool2 = pool.clone();
    let from_db = warp::path!("hello" / i32)
        .and(warp::any().map(move || pool2.clone()))
        .and_then(handle_from_db);

    //let user_handling = warp::post().and(warp::path("/login").map(|| "awesome"));
    warp::serve(
        users::register_route(pool.clone())
            .or(users::login_route(pool.clone()))
            .or(warp::get().and(from_db).or(hello))
            .or(warp::get()
                .and(warp::path("test"))
                .and(force_logged_in(pool))
                .map(|v| format!("awesome? {}", v)))
            .recover(handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(error) = err.find::<ReturnErrors>() {
        let res = handle_custom_error(error);
        code = res.0;
        message = res.1;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".into();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorMessage {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
fn handle_custom_error(error: &ReturnErrors) -> (StatusCode, String) {
    match error {
        ReturnErrors::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "file not found".into()),
        ReturnErrors::GenericError(x) => (StatusCode::INTERNAL_SERVER_ERROR, x.to_owned()),
        ReturnErrors::DatabaseError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something wend wrong".into(),
        ),
        ReturnErrors::NotFound => (StatusCode::NOT_FOUND, "resource not found".into()),
        ReturnErrors::HashError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something wend wrong".into(),
        ),
        ReturnErrors::CustomError(message, code) => (*code, message.to_string()),
    }
}
