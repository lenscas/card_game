use card_game::controllers::{self, users::force_logged_in};
use card_game::errors::ReturnError;
use card_game::util::CastRejection;
use card_game_shared::ErrorMessage;
use dotenv::{dotenv, var};
use sqlx::{query, PgPool};
use std::{convert::Infallible, net::SocketAddr};
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

async fn handle_from_db(
    id: i64,
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
    pretty_env_logger::init();
    dotenv().unwrap();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let binding_address = var("BINDING_ADDRESS")
        .expect("BINDING_ADDRESS is not set.")
        .parse()
        .expect("Malformed ip address");
    let port = var("PORT")
        .expect("PORT is not set")
        .parse()
        .expect("Invalid port");
    let addr = SocketAddr::new(binding_address, port);
    println!("Hello, world!");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Couldn't connect to database");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let pool2 = pool.clone();
    let from_db = warp::path!("hello" / i64)
        .and(warp::any().map(move || pool2.clone()))
        .and_then(handle_from_db);

    use http::method::Method;
    let cors = warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "content-type",
            "authorization_token",
        ])
        .allow_methods(
            vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ]
            .into_iter(),
        );

    warp::serve(
        warp::any()
            .and(
                controllers::users::user_route(pool.clone())
                    .or(warp::path("assets").and(warp::fs::dir("assets")))
                    .or(controllers::characters::character_routes(pool.clone()))
                    .or(controllers::battle::battle_route(pool.clone()))
                    .or(controllers::dungeon::dungeon_routes(pool.clone()))
                    .or(warp::get().and(from_db).or(hello))
                    .or(warp::get()
                        .and(warp::path("test"))
                        .and(force_logged_in(pool))
                        .map(|v| format!("awesome? {}", v)))
                    .recover(handle_rejection),
            )
            .with(cors),
    )
    .run(addr)
    .await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let err = dbg!(err);
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(error) = err.find::<ReturnError>() {
        let res = handle_custom_error(error);
        let res = match res {
            ReturnHandle::Custom(x) => return Ok(x),
            ReturnHandle::Parts(x, y) => (x, y),
        };
        code = res.0;
        message = res.1;
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
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

    let json = warp::reply::json(&ErrorMessage { message });

    Ok(Box::new(warp::reply::with_status(json, code)))
}

enum ReturnHandle {
    Custom(Box<dyn Reply>),
    Parts(StatusCode, String),
}

impl ReturnHandle {
    fn new(a: StatusCode, b: String) -> Self {
        Self::Parts(a, b)
    }
}
impl<T: Reply + 'static> From<T> for ReturnHandle {
    fn from(a: T) -> Self {
        Self::Custom(Box::new(a))
    }
}

fn handle_custom_error(error: &ReturnError) -> ReturnHandle {
    match error {
        ReturnError::Io(_) => {
            ReturnHandle::new(StatusCode::INTERNAL_SERVER_ERROR, "file not found".into())
        }
        ReturnError::GenericError(x) => {
            ReturnHandle::new(StatusCode::INTERNAL_SERVER_ERROR, x.to_owned())
        }

        ReturnError::NotFound => {
            ReturnHandle::new(StatusCode::NOT_FOUND, "resource not found".into())
        }
        ReturnError::HashError(_) => {
            ReturnHandle::new(StatusCode::INTERNAL_SERVER_ERROR, "in custom error".into())
        }
        ReturnError::CustomError(message, code) => ReturnHandle::new(*code, message.to_string()),
        ReturnError::BattleErrors(x) => {
            serde_json::to_string(&card_game_shared::battle::TurnResponse::Error(x.to_owned()))
                .map(|v| warp::reply::with_status(v, StatusCode::CONFLICT).into())
                .unwrap_or_else(|v| handle_custom_error(&ReturnError::from(v)))
        }
        ReturnError::DatabaseError(_) | ReturnError::LuaError(_) | ReturnError::JsonError(_) => {
            ReturnHandle::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "something wend wrong".into(),
            )
        }
    }
}
