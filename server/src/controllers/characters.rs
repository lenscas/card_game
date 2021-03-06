use super::users::{force_logged_in, with_db};
use crate::{dungeon::Dungeon, errors::ReturnError, util::convert_error};
use card_game_shared::characters::{CharacterCreationResponse, CharacterList};
use futures::StreamExt;
use sqlx::{query, PgPool};
use warp::{Filter, Reply};

pub(crate) async fn create_character(
    user_id: i64,
    db: PgPool,
) -> Result<Box<dyn Reply>, ReturnError> {
    let mut con = db.begin().await?;
    let res = query!(
        "SELECT count(id) FROM characters WHERE user_id = $1",
        user_id
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
        "INSERT INTO characters (user_id, dungeon, character_state) VALUES ($1, $2,$3) RETURNING id",
        user_id,
        serde_json::to_value(Dungeon::new(10,10))?,
        serde_json::from_str::<serde_json::Value>("{}")?
    )
    .fetch_one(&mut con)
    .await?
    .id;
    let deck_id = query!(
        "INSERT INTO decks (character_id) VALUES ($1) RETURNING id",
        id
    )
    .fetch_one(&mut con)
    .await?
    .id;
    query!(
        "INSERT INTO cards_in_deck (deck_id,card_id)
        SELECT $1, card_id
        FROM owned_starting_cards
        WHERE owned_starting_cards.user_id = $2",
        deck_id,
        i64::from(user_id)
    )
    .execute(&mut con)
    .await?;

    con.commit().await?;

    Ok(Box::new(serde_json::to_string(
        &CharacterCreationResponse { id },
    )?))
}

pub(crate) async fn get_characters(id: i64, db: PgPool) -> Result<Box<dyn Reply>, ReturnError> {
    let mut con = db.begin().await?;
    let res = query!("SELECT id FROM characters WHERE user_id = $1", id)
        .fetch(&mut con)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|v| v.map(|v| v.id))
        .collect::<Result<Vec<_>, _>>()?;

    let list = CharacterList { characters: res };

    con.commit().await?;
    Ok(Box::new(serde_json::to_string(&list)?))
}

pub(crate) async fn get_character_in_battle(
    user_id: i64,
    char_id: i64,
    db: PgPool,
) -> Result<Box<dyn Reply>, ReturnError> {
    let count = query!(
        "
        SELECT COUNT(*) as count 
        FROM characters 
        WHERE user_id=$1 
        AND id=$2 
        AND current_battle IS NULL",
        user_id,
        char_id
    )
    .fetch_one(&db)
    .await?
    .count
    .expect("count was not set for some reason?");
    Ok(Box::new(warp::reply::json(&(count > 0))))
}

pub fn character_routes(
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
                .and(force_logged_in(db.clone()))
                .and(warp::path::param::<i64>())
                .and(with_db(db.clone()))
                .and_then(|user_id, char_id, conn| {
                    convert_error((user_id, char_id, conn), |(a, b, c)| {
                        get_character_in_battle(a, b, c)
                    })
                }),
        ))
        .or(warp::get().and(
            warp::path("characters")
                .and(with_db(db.clone()))
                .and(force_logged_in(db))
                .and_then(|a, b| convert_error((a, b), |(b, a)| get_characters(a, b))),
        ))
}
