use axum::{extract::Form, response::IntoResponse, Router, routing, Json};
use serde::Deserialize;

use crate::database::Repositories;
use crate::views::{Home, Tweet};

pub fn tweets(repos: Repositories) -> Router {
    Router::new().route("/new", routing::post(post))
    .with_state(repos)
}

async fn post(form: Form<TweetForm>) -> impl IntoResponse {
    let tweets = vec![Tweet {
        name: "太郎".to_string(),
        message: form.message.to_string(),
        posted_at: "2020-01-01 12:34".to_string(),
    }];
    let home = Home { tweets };
    Json(home)
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}