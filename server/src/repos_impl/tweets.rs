use std::sync::Arc;

use crate::database::ConnectionPool;
use crate::entities::Tweet;
use crate::repositories::Tweets;

#[derive(Clone)]
pub struct TweetsInpl {
    pub pool: Arc<ConnectionPool>,
}

#[axum::async_trait]
impl Tweets for TweetsImpl {
    async fn list(&self) -> Vec<Tweet> {
        let conn = self.pool.get().await.unwrap();
        let rows = conn
            .query("SELECT * FROM tweets ORDER BY posted_at DESC", &[])
            .await
            .unwrap();
        rows.into_iter()
            .map(|r| Tweet::new(r.get("id"), r.get("message"), r.get("posted_at")))
            .collect()
    }
}