use super::users::force_logged_in;
use crate::{battle::Field, controllers::users::with_db, errors::ReturnError, util::convert_error};
use card_game_shared::battle::{TakeAction, TurnResponse};
use sqlx::{query, PgPool};
use warp::{path::param, Filter, Reply};

async fn get_battle(
    db: PgPool,
    user_id: i64,
    character_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    Ok(Box::new(warp::reply::json(
        &Field::get_from_db(user_id, character_id, &db)
            .await?
            .into_shared(),
    )))
}

async fn do_turn(
    action: TakeAction,
    db: PgPool,
    user_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    let chosen_card = action.play_card;
    let mut con = db.begin().await?;
    let battle = Field::get_from_db(user_id, action.character_id, &mut con).await?;
    let (battle, event_list, is_over) = battle.process_turn(chosen_card).await?;
    if is_over {
        query!(
            "UPDATE characters SET current_battle = null WHERE user_id = $1 and id=$2",
            user_id,
            action.character_id
        )
        .execute(&mut con)
        .await?;
        con.commit().await?;
        return Ok(Box::new(serde_json::to_string(&TurnResponse::Done)?));
    } else {
        battle.save(user_id, action.character_id, &mut con).await?;
    }
    con.commit().await?;
    Ok(Box::new(warp::reply::json(&TurnResponse::NextTurn(
        event_list,
    ))))
}

pub fn battle_route(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{get, path, post};
    path("battle")
        .and(
            post()
                .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
                .and(with_db(db.clone()))
                .and(force_logged_in(db.clone()))
                .and_then(|action, pool, id| {
                    convert_error((action, pool, id), |(action, pool, id)| {
                        do_turn(action, pool, id)
                    })
                }),
        )
        .or(get()
            .and(with_db(db.clone()))
            .and(force_logged_in(db))
            .and(path("battle"))
            .and(param::<i64>())
            .and(path::end())
            .and_then(|db, user_id, character_id| {
                convert_error(
                    (db, user_id, character_id),
                    |(db, user_id, character_id)| get_battle(db, user_id, character_id),
                )
            }))
        .boxed()
}
