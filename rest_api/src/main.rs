use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
    user: User,
}

use warp::{http::StatusCode, Filter};

async fn create_user(user: User) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = Response {
        message: "User created successfully".to_string(),
        user,
    };
    Ok(warp::reply::json(&reply))
}

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("user"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(create_user)
}

#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}