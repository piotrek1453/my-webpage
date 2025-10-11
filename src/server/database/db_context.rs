use crate::server::prelude::*;

pub struct DbContext {
    pub pool: Pool<Postgres>,
}

impl DbContext {
    pub fn new() -> Result<Self, ServerFnError> {
        let pool = use_context::<Pool<Postgres>>()
            .ok_or_else(|| ServerFnError::<Infallible>::ServerError("DB pool missing".into()))?;
        Ok(Self { pool })
    }

    pub async fn get_posts(&self) -> Result<Vec<post::Post>, sqlx::Error> {
        post::Post::get_all(&self.pool).await
    }

    pub async fn create_post(&self, post: post::Post) -> Result<(), sqlx::Error> {
        post.insert(&self.pool).await
    }
}
