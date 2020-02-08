use dotenv::{dotenv, var};
use futures::future::*;
use sqlx::{query, PgPool};
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set.");
    println!("Hello, world!");

    let pool = PgPool::new(&db_url)
        .await
        .expect("Couldn't connect to database");
    /*
    let v = query!("SELECT id,username,password FROM users WHERE id = $1", 1)
        .fetch_all(&mut pool)
        .await
        .unwrap();
    v.iter().for_each(|rec| {
        println!(
            "id : {:0}, username : {:1}, password : {:2}",
            rec.id, rec.username, rec.password
        )
    });*/
    let x = || pool.clone();
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let from_db = warp::path!("hello" / i32).map(|id| {
        let bla = id;
        async {
            let mut con = x().try_acquire().unwrap();
            query!("SELECT username FROM users WHERE id = $1", 1)
                .fetch_one(&mut con)
                .map(|v| match v {
                    Ok(v) => Ok(v.username),
                    Err(_) => Err(warp::reject::not_found()),
                })
        }
    });
    //.map(|name: String| format!("Hello from db {}", name));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
