use super::users::{force_logged_in, with_db};
use crate::{battle::Field, dungeon::Dungeon, errors::ReturnError, util::convert_error};
use card_game_shared::dungeon::EventProcesed;
use sqlx::PgPool;
use warp::{http::StatusCode, Filter, Reply};

pub(crate) async fn get_dungeon(
    db: PgPool,
    user_id: i64,
    character_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    Ok(Box::new(warp::reply::json(
        &Dungeon::select_from_db(user_id, character_id, &db)
            .await?
            .to_shared(),
    )))
}

pub(crate) async fn do_move(
    db: PgPool,
    user_id: i64,
    character_id: i64,
    move_to: card_game_shared::BasicVector<isize>,
) -> Result<Box<dyn Reply>, ReturnError> {
    let mut transaction = db.begin().await?;
    let dungeon =
        Dungeon::select_from_db_no_battle(user_id, character_id, &mut transaction).await?;
    let mut dungeon = match dungeon {
        Some(x) => x,
        None => {
            return Err(ReturnError::CustomError(
                serde_json::to_string(&EventProcesed::CurrentlyInBattle)?,
                StatusCode::CONFLICT,
            ))
        }
    };
    let did_move = dungeon.try_move(move_to);
    match did_move {
        Some(entered_fight) => {
            dungeon
                .save(user_id, character_id, &mut transaction)
                .await?;
            if entered_fight {
                let field = Field::new(user_id, character_id, &mut transaction).await?;
                field.save(user_id, character_id, &mut transaction).await?;
            }
            transaction.commit().await?;
            Ok(Box::new(warp::reply::json(&EventProcesed::Success(
                entered_fight,
            ))))
        }
        None => Err(ReturnError::CustomError(
            serde_json::to_string(&EventProcesed::Error)?,
            StatusCode::BAD_REQUEST,
        )),
    }
}

pub fn dungeon_routes(
    db: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::{get, path, post};
    path("dungeon")
        .and(
            get()
                .and(with_db(db.clone()))
                .and(force_logged_in(db.clone()))
                .and(warp::path::param::<i64>())
                .and_then(|db, user_id: i64, character_id: i64| {
                    convert_error(
                        (db, user_id, character_id),
                        |(db, user_id, character_id)| get_dungeon(db, user_id, character_id),
                    )
                }),
        )
        .or(post()
            .and(with_db(db.clone()))
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and(force_logged_in(db))
            .and(warp::path("dungeon"))
            .and(warp::path::param::<i64>())
            .and(path("move"))
            .and(warp::path::end())
            .and_then(
                |db,
                 move_to: card_game_shared::BasicVector<isize>,
                 user_id: i64,
                 character_id: i64| {
                    convert_error(
                        (db, user_id, character_id, move_to),
                        |(db, user_id, character_id, move_to)| {
                            do_move(db, user_id, character_id, move_to)
                        },
                    )
                },
            ))
        .boxed()
}
