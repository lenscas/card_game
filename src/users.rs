use dotenv::var;
use sqlx::{pool::PoolConnection, PgConnection};
use sqlx::{query, PgPool};
use warp::Filter;
use warp::{reject::Rejection, Reply};

use serde_derive::Deserialize;

use crate::errors::ReturnErrors;
use argonautica::{Hasher, Verifier};

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
    password_check: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub async fn get_db_con(db: PgPool) -> Result<PoolConnection<PgConnection>, Rejection> {
    match db.acquire().await {
        Ok(x) => Ok(x),
        Err(x) => Err(warp::reject::custom::<ReturnErrors>(x.into())),
    }
}

fn json_body_register() -> impl Filter<Extract = (RegisterData,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_login() -> impl Filter<Extract = (LoginData,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn login(req_data: LoginData, db: PgPool) -> Result<impl Reply, Rejection> {
    let mut con = get_db_con(db).await?;
    match query!(
        "SELECT id,password FROM users where username = $1",
        req_data.username
    )
    .fetch_one(&mut con)
    .await
    {
        Ok(user) => {
            let v = Verifier::default()
                .with_secret_key(var("PEPPER").unwrap())
                .with_password(req_data.password)
                .with_hash(user.password)
                .verify();
            match v {
                Ok(res) => {
                    if res {
                        Ok("Logged in correctly!")
                    } else {
                        Err(warp::reject::reject())
                    }
                }
                Err(x) => {
                    dbg!(x);
                    Err(warp::reject::reject())
                }
            }
        }
        Err(x) => {
            dbg!(x);
            Err(warp::reject::reject())
        }
    }

    //return Ok("awesome");
}

pub async fn register(reg_data: RegisterData, db: PgPool) -> Result<impl Reply, Rejection> {
    if reg_data.password != reg_data.password_check {
        return Err(warp::reject::reject());
    }
    let mut con = get_db_con(db).await?;
    let hashed_password = Hasher::default()
        .with_password(reg_data.password)
        .with_secret_key(var("PEPPER").unwrap())
        .hash();
    let hashed_password = match hashed_password {
        Ok(x) => Ok(x),
        Err(x) => {
            dbg!(x);
            Err(warp::reject::reject())
        }
    }?;
    let v = query!(
        "INSERT INTO users ( username, password ) VALUES ( $1 , $2 )",
        reg_data.username,
        hashed_password
    )
    .execute(&mut con)
    .await;
    match v {
        Ok(_) => Ok("Account created!"),
        Err(x) => {
            dbg!(x);
            Err(warp::reject::reject())
        }
    }
    //return Ok("Awesome!");
}

fn with_db(
    db: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn register_route(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{path, post};
    post().and(
        path("register")
            .and(json_body_register())
            .and(with_db(db))
            .and_then(register),
    )
}
pub fn login_route(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{path, post};
    post().and(
        path("login")
            .and(json_body_login())
            .and(with_db(db))
            .and_then(login),
    )
}
