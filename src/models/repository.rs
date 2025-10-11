use async_trait::async_trait;
use sqlx::{Pool, Postgres, Result};

#[async_trait]
pub trait Repository {
    type Entity;
    async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Self::Entity>>;
    async fn get_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Option<Self::Entity>>;
    async fn insert(&self, pool: &Pool<Postgres>) -> Result<()>;
    async fn update(&self, pool: &Pool<Postgres>) -> Result<()>;
    async fn delete(pool: &Pool<Postgres>, id: i64) -> Result<()>;
}
