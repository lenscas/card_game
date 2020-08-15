use super::users::force_logged_in;
use crate::{battle::Field, controllers::users::with_db, errors::ReturnError, util::convert_error};
use card_game_shared::battle::{ReturnBattle, TakeAction, TurnResponse};
use sqlx::{query, PgPool};
use warp::{Filter, Reply};

async fn create_battle(
    db: PgPool,
    user_id: i32,
    character_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    println!("???");
    let user_id = i64::from(user_id);

    let mut con = db.begin().await?;

    query!(
        "UPDATE characters
        SET current_battle = NULL 
        WHERE user_id = $1
        AND characters.id = $2",
        user_id,
        character_id
    )
    .execute(&mut con)
    .await?;
    let v =query!(
        "SELECT COUNT(*) AS count FROM characters WHERE user_id = $1 AND current_battle IS NOT NULL",
        user_id
    )
    .fetch_one(&mut con)
    .await?;
    let conflict_error = Err(ReturnError::CustomError(
        "Already in battle".into(),
        warp::http::StatusCode::CONFLICT,
    ));
    if v.count.unwrap() == 1 {
        return conflict_error;
    }
    let battle = Field::new(user_id, &mut con).await?;
    let as_str = serde_json::to_string(&battle)?;
    let rows = query!(
        "UPDATE characters SET current_battle=$1 WHERE user_id=$2 AND current_battle IS NULL",
        as_str,
        user_id
    )
    .execute(&mut con)
    .await?;
    if rows != 1 {
        return conflict_error;
    }
    let hand = battle.player.deck.get_ids_from_hand();
    con.commit().await?;
    Ok(Box::new(serde_json::to_string(&ReturnBattle {
        player_hp: battle.player.life,
        enemy_hp: battle.ai.life,
        enemy_hand_size: battle.ai.deck.hand.len(),
        success: true,
        hand,
        enemy_mana: battle.ai.mana,
        mana: battle.player.mana,
        hexa_runes: battle
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
        small_runes: battle
            .player
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
        enemy_small_runes: battle
            .ai
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
    })?))
}

async fn do_turn(
    action: TakeAction,
    db: PgPool,
    user_id: i32,
) -> Result<Box<dyn Reply>, ReturnError> {
    let chosen_card = action.play_card;
    let user_id = i64::from(user_id);
    let mut con = db.begin().await?;
    let v = query!(
        "SELECT current_battle 
        FROM characters 
        WHERE user_id = $1 
        AND current_battle IS NOT NULL
        AND characters.id = $2",
        user_id,
        action.character_id
    )
    .fetch_one(&mut con)
    .await?;
    let battle: Field = serde_json::from_str(&v.current_battle.unwrap())?;
    let (battle, is_over) = battle.process_turn(chosen_card).await?;
    if is_over {
        query!(
            "UPDATE characters SET current_battle = null WHERE user_id = $1",
            user_id
        )
        .execute(&mut con)
        .await?;
        return Ok(Box::new(serde_json::to_string(&TurnResponse::Done)?));
    } else {
        let c = serde_json::to_string(&battle)?;
        query!(
            "UPDATE characters SET current_battle = $1 WHERE user_id = $2",
            c,
            user_id
        )
        .execute(&mut con)
        .await?;
    }

    let hand = battle.player.deck.get_ids_from_hand();
    let hand = dbg!(hand);
    con.commit().await?;
    Ok(Box::new(serde_json::to_string(&ReturnBattle {
        player_hp: battle.player.life,
        enemy_hp: battle.ai.life,
        enemy_hand_size: battle.ai.deck.hand.len(),
        success: true,
        hand,
        enemy_mana: battle.ai.mana,
        mana: battle.player.mana,
        hexa_runes: battle
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
        small_runes: battle
            .player
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
        enemy_small_runes: battle
            .ai
            .runes
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.name.clone())
            .collect(),
    })?))
}

pub fn battle_route(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{path, post, put};
    path("battle")
        .and(
            post()
                .and(with_db(db.clone()))
                .and(force_logged_in(db.clone()))
                .and(warp::path::param::<i64>())
                .and_then(|db, user_id: i32, character_id: i64| {
                    convert_error(
                        (db, user_id, character_id),
                        |(db, user_id, character_id)| create_battle(db, user_id, character_id),
                    )
                }),
        )
        .or(put()
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and(with_db(db.clone()))
            .and(force_logged_in(db))
            .and_then(|action, pool, id| {
                convert_error((action, pool, id), |(action, pool, id)| {
                    do_turn(action, pool, id)
                })
            }))
        .boxed()
}