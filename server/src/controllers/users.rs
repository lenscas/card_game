use crate::util::{convert_error, CastRejection};
use card_game_shared::users::{LoginData, LoginReply, RegisterData};
use dotenv::var;
use sqlx::{query, PgPool};
use warp::Filter;
use warp::Reply;

use crate::errors::ReturnError;
use argonautica::{Hasher, Verifier};

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

pub(crate) async fn login(req_data: LoginData, db: PgPool) -> Result<impl Reply, ReturnError> {
    let mut con = db.begin().await?;
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
            ReturnError::CustomError(
                "{\"success\":false}".into(),
                warp::http::StatusCode::NOT_FOUND,
            )
        })
    })?;

    let v = Verifier::default()
        .with_secret_key(var("PEPPER").unwrap())
        .with_password(req_data.password)
        .with_hash(user.password)
        .verify()?;
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
                    con.commit().await?;
                    return Ok(warp::reply::json(&LoginReply {
                        success: true,
                        token,
                    }));
                }
                Err(x) => match x {
                    Error::Database(x) => {
                        dbg!(x);
                    }
                    x => {
                        let x = dbg!(x);
                        return Err(x.into());
                    }
                },
            }
        }
        Err(ReturnError::CustomError(
            "{{\"success\":false}}".into(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Err(ReturnError::CustomError(
            "{\"success\":false}".into(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub(crate) async fn register(
    reg_data: RegisterData,
    con: PgPool,
) -> Result<impl Reply, ReturnError> {
    if reg_data.password != reg_data.password_check {
        return Err(ReturnError::CustomError(
            "Password does not match password check".into(),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    let hashed_password = Hasher::default()
        .with_password(reg_data.password)
        .with_secret_key(var("PEPPER").unwrap())
        .hash()?;
    let mut con = con.begin().await?;
    let v = query!(
        "INSERT INTO users ( username, password ) VALUES ( $1 , $2 ) RETURNING id",
        reg_data.username,
        hashed_password
    )
    .fetch_one(&mut con)
    .await?;

    query!(
        "INSERT INTO owned_starting_cards
            SELECT
                $1 as user_id,
                nextval('owned_starting_cards_id_seq') as id,
                id as card_id
            FROM cards
            WHERE is_starting_card = true
            ",
        v.id as i64
    )
    .execute(&mut con)
    .await?;
    con.commit().await?;
    Ok("Account created".to_string())
}

pub fn with_db(
    db: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn user_route(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{path, post};
    post()
        .and(
            path("login")
                .and(json_body_login())
                .and(with_db(db.clone()))
                .and_then(|data, db| convert_error((data, db), |(data, db)| login(data, db))),
        )
        .or(path("register")
            .and(json_body_register())
            .and(with_db(db))
            .and_then(|a, db| convert_error((a, db), |(a, db)| register(a, db))))
}

pub fn force_logged_in(
    db: PgPool,
) -> impl Filter<Extract = (i64,), Error = warp::Rejection> + Clone {
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
