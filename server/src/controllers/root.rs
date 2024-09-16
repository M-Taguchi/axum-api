use axum::{extract::State, response::IntoResponse, Router, routing, Json};

use crate::controllers::tweets;
use crate::database;
use crate::repos_impl::TweetsImpl;
use crate::services::list_tweets;

pub async fn app() -> Router {
    let repos = database::resolve_repositories().await;
    Router::new().
        route("/", routing::get(get))
        .with_state(repos.clone())
        .nest("/tweets", tweets(repos.clone()))
}

async fn get(State(tweets_repo): State<TweetsImpl>) -> impl IntoResponse {
    let home = list_tweets(&tweets_repo).await;
    Json(home)
}