use axum::{
    routing::{get, post},
    Router
};
use tokio::fs::create_dir;

mod routes;

use crate::routes::{status, artifacts,team,user};

#[tokio::main]
async fn main() {
    env_logger::init();

    create_dir(std::path::Path::new(artifacts::v8::CACHE_DIR)).await;

    let app = Router::new()
    .route("/status", get(status))
    .route("/v2/team", get(team::v2::team))
    .route("/v2/teams", get(team::v2::teams))
    .route("/v2/user", get(user::v2::user))
    .route("/v8/artifacts/:id", get(artifacts::v8::get).put(artifacts::v8::put))
    .route("/v8/artifacts/events", post(artifacts::v8::events))
    .route("/v8/artifacts/status", get(artifacts::v8::status));


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}