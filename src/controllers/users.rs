use crate::util::CastRejection;
use card_game_shared::{LoginData, LoginReply, RegisterData};
use dotenv::var;
use sqlx::{pool::PoolConnection, PgConnection};
use sqlx::{query, PgPool};
use warp::Filter;
use warp::{reject::Rejection, Reply};

use crate::errors::ReturnErrors;
use argonautica::{Hasher, Verifier};

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
    let user = query!(
        "SELECT id,password FROM users where username = $1",
        req_data.username
    )
    .fetch_one(&mut con)
    .await
    .half_cast()
    .map_err(|err| {
        let err = dbg!(err);
        err.map_not_found(|| {
            ReturnErrors::CustomError(
                "{\"success\":false}".into(),
                warp::http::StatusCode::NOT_FOUND,
            )
        })
    })?;

    let v = Verifier::default()
        .with_secret_key(var("PEPPER").unwrap())
        .with_password(req_data.password)
        .with_hash(user.password)
        .verify()
        .cast()?;
    if v {
        use base64::encode;
        use rand_core::{OsRng, RngCore};
        use sha2::{Digest, Sha256};
        use sqlx::Error;
        use std::time::SystemTime;
        let secret = var("LOGIN_TOKEN_SECRET").unwrap();
        for _ in 0..3 {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let mut bytes = [0u8, 16];
            OsRng.fill_bytes(&mut bytes);
            let token = format!(
                "{}/{}/{}",
                now,
                bytes.iter().map(|v| v.to_string()).collect::<String>(),
                secret
            );
            let token = Sha256::digest(token.as_bytes());
            let token = encode(&token);

            let success = query!(
                "INSERT INTO sessions ( hash,user_id) VALUES ($1,$2)",
                token,
                user.id
            )
            .execute(&mut con)
            .await;
            match success {
                Ok(_) => {
                    return Ok(warp::reply::json(&LoginReply {
                        success: true,
                        token,
                    }))
                }
                Err(x) => match x {
                    Error::Database(x) => {
                        dbg!(x);
                    }
                    x => {
                        let x = dbg!(x);
                        return Err(x).cast();
                    }
                },
            }
        }
        Err(ReturnErrors::CustomError(
            "{{\"success\":false}}".into(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
        .cast()
    } else {
        Err(ReturnErrors::CustomError(
            "{\"success\":false}".into(),
            warp::http::StatusCode::NOT_FOUND,
        )
        .into())
    }
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

pub fn with_db(
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

pub fn force_logged_in(
    db: PgPool,
) -> impl Filter<Extract = (i32,), Error = warp::Rejection> + Clone {
    warp::header::header("authorization_token")
        .and(with_db(db))
        .and_then(|token: String, db: PgPool| async move {
            let token = dbg!(token);
            let mut con = db.acquire().await.cast()?;
            query!("SELECT user_id FROM sessions WHERE hash = $1", token)
                .fetch_one(&mut con)
                .await
                .map(|v| v.user_id)
                .cast()
        })
}
