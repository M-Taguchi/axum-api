use crate::views::partial::Tweet;
use serde::Serialize;

#[derive(Serialize)]
pub struct Home {
    pub tweets: Vec<Tweet>,
}