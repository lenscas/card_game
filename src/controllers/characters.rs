use super::users::{force_logged_in, with_db};
use crate::{errors::ReturnError, util::convert_error};
use card_game_shared::characters::{CharacterCreationResponse, CharacterList};
use sqlx::{query, PgPool};
use tokio::stream::StreamExt;
use warp::{Filter, Reply};

pub(crate) async fn create_character(id: i32, db: PgPool) -> Result<Box<dyn Reply>, ReturnError> {
    let mut con = db.begin().await?;
    let res = query!(
        "SELECT count(id) FROM characters WHERE user_id = $1",
        id as i64
    )
    .fetch_one(&mut con)
    .await?;
    if res.count.unwrap() > 1 {
        return Err(ReturnError::custom(
            "There is already a character active",
            warp::http::StatusCode::CONFLICT,
        ));
    }
    let id = query!(
        "INSERT INTO characters (user_id) VALUES ($1) RETURNING id",
        id as i64
    )
    .fetch_one(&mut con)
    .await?
    .id;
    con.commit().await?;
    Ok(Box::new(serde_json::to_string(
        &CharacterCreationResponse { id },
    )?))
}

pub(crate) async fn get_characters(id: i32, db: PgPool) -> Result<Box<dyn Reply>, ReturnError> {
    let mut con = db.begin().await?;
    let res = query!("SELECT id FROM characters WHERE user_id = $1", id as i64)
        .fetch(&mut con)
        .map(|v| v.map(|v| v.id))
        .collect::<Result<_, _>>()
        .await?;
    let list = CharacterList { characters: res };

    con.commit().await?;
    Ok(Box::new(serde_json::to_string(&list)?))
}

pub(crate) fn character_routes(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .and(
            warp::path("characters")
                .and(warp::post())
                .and(with_db(db.clone()))
                .and(force_logged_in(db.clone()))
                .and_then(|a, b| convert_error((a, b), |(a, b)| create_character(b, a))),
        )
        .or(warp::get().and(
            warp::path("characters")
                .and(with_db(db.clone()))
                .and(force_logged_in(db))
                .and_then(|a, b| convert_error((a, b), |(b, a)| get_characters(a, b))),
        ))
}
