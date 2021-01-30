use super::users::{force_logged_in, with_db};
use crate::{battle::Field, dungeon::Dungeon, errors::ReturnError, util::convert_error};
use card_game_shared::{dungeon::EventProcesed, image_map::ImageUrlWithName};
use image::{open, png::PngEncoder, EncodableLayout, Rgba};
use sqlx::{query, PgPool};
use tokio::fs::{read_dir, DirEntry};
use warp::{http::StatusCode, path::end, Filter, Reply};

pub(crate) async fn get_dungeon(
    db: PgPool,
    user_id: i64,
    character_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    println!("got here?");
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
async fn get_all_tiles() -> std::io::Result<Vec<DirEntry>> {
    let mut stream = read_dir("./assets/tiles/").await?;
    let mut files = Vec::new();
    while let Some(entry) = stream.next_entry().await? {
        if let Some(x) = entry.path().extension() {
            if x == "png" {
                files.push(entry)
            }
        }
    }
    Ok(files)
}
pub async fn get_tile_list(_: i64) -> Result<Box<dyn Reply>, ReturnError> {
    let files = get_all_tiles().await?;
    Ok(Box::new(warp::reply::json(
        &files
            .iter()
            .filter_map(|v| v.file_name().to_str().map(ToOwned::to_owned))
            .map(|v| ImageUrlWithName {
                name: v.clone(),
                url: format!("/assets/tiles/{}", v),
            })
            .collect::<Vec<_>>(),
    )))
}
pub async fn generate_dungeon_tile_map(
    db: PgPool,
    user_id: i64,
) -> Result<Box<dyn Reply>, ReturnError> {
    let path = get_all_tiles().await?;
    let x = path
        .iter()
        .map(|v| open(v.path()))
        .filter_map(|v| match v {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .map(|v| {
            let v = v.into_rgba8();
            let dimensions = v.dimensions();
            let bytes: Vec<_> = v.pixels().flat_map(|v| v.0.iter()).copied().collect();
            sheep::InputSprite { bytes, dimensions }
        })
        .collect::<Vec<_>>();
    let x = sheep::pack::<sheep::SimplePacker>(x, 4, ()).remove(0);
    let encoding = sheep::encode::<card_game_shared::image_map::ImageFormat>(&x, ());
    let encoding = serde_json::to_value(encoding)?;
    query!(
        "UPDATE users
        SET dungeon_tile_map = $1
        WHERE id=$2",
        encoding,
        user_id
    )
    .execute(&db)
    .await?;
    let tiled_image =
        image::ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(x.dimensions.0, x.dimensions.1, x.bytes)
            .unwrap();
    let mut buffer = Vec::with_capacity(500_000);
    PngEncoder::new(&mut buffer)
        .encode(
            tiled_image.as_bytes(),
            tiled_image.dimensions().0,
            tiled_image.dimensions().1,
            image::ColorType::Rgba8,
        )
        .unwrap();
    Ok(Box::new(warp::reply::with_header(
        buffer,
        "Content-Type",
        "image/png",
    )))
}

pub async fn get_dungeon_tile_map(db: PgPool, user_id: i64) -> Result<Box<dyn Reply>, ReturnError> {
    let res = sqlx::query!("SELECT dungeon_tile_map FROM users WHERE id=$1", user_id)
        .fetch_one(&db)
        .await?
        .dungeon_tile_map;
    let x = res
        .map(serde_json::from_value::<card_game_shared::image_map::SerializedSpriteSheet>)
        .ok_or_else(|| {
            ReturnError::CustomError(
                String::from("No tilemap generated yet"),
                warp::http::StatusCode::NOT_FOUND,
            )
        })??;
    Ok(Box::new(warp::reply::json(&x)))
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
        .or(get()
            .and(path("dungeon").and(path("tiles").and(end())))
            .and(with_db(db.clone()))
            .and(force_logged_in(db.clone()))
            .and_then(|db, user_id| {
                convert_error((db, user_id), |(db, user_id)| {
                    generate_dungeon_tile_map(db, user_id)
                })
            }))
        .or(get()
            .and(
                path("dungeon")
                    .and(path("tiles"))
                    .and(path("list"))
                    .and(end()),
            )
            .and(force_logged_in(db.clone()))
            .and_then(|user_id| convert_error(user_id, get_tile_list)))
        .or(get()
            .and(
                path("dungeon")
                    .and(path("tiles"))
                    .and(path("map"))
                    .and(end()),
            )
            .and(with_db(db.clone()))
            .and(force_logged_in(db.clone()))
            .and_then(|db, user_id| {
                convert_error((db, user_id), |(db, user_id)| {
                    get_dungeon_tile_map(db, user_id)
                })
            }))
        .or(post()
            .and(with_db(db.clone()))
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and(force_logged_in(db))
            .and(warp::path("dungeon"))
            .and(warp::path::param::<i64>())
            .and(path("move"))
            .and(end())
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
