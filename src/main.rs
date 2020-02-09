use dotenv::{dotenv, var};
use sqlx::{query, PgPool};
use warp::Filter;

mod users;

async fn handle_from_db(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let mut con = pool.acquire().await.unwrap();
    match query!("SELECT username FROM users WHERE id = $1", id)
        .fetch_one(&mut con)
        .await
    {
        Ok(v) => Ok(v.username),
        Err(err) => {
            use sqlx::Error::*;
            match err {
                NotFound => Err(warp::reject::not_found()),
                e => {
                    dbg!(e);
                    Err(warp::reject::reject())
                }
            }
        }
    }
}
#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set.");
    println!("Hello, world!");

    let pool = PgPool::new(&db_url)
        .await
        .expect("Couldn't connect to database");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let pool2 = pool.clone();
    let from_db = warp::path!("hello" / i32)
        .and(warp::any().map(move || pool2.clone()))
        .and_then(handle_from_db);

    //let user_handling = warp::post().and(warp::path("/login").map(|| "awesome"));
    warp::serve(
        users::register_route(pool.clone())
            .or(users::login_route(pool.clone()))
            .or(warp::get().and(from_db).or(hello)),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}
