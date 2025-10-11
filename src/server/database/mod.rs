pub use crate::models::*;
pub use leptos::prelude::use_context;
#[cfg(feature = "ssr")]
pub mod db_context;

#[cfg(feature = "ssr")]
pub mod repo {
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

    use crate::models::post::Post;

    #[async_trait]
    impl Repository for Post {
        type Entity = Post;

        async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Self::Entity>> {
            sqlx::query_as!(
                Post,
                r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post"#
            )
            .fetch_all(pool)
            .await
        }

        async fn get_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Option<Self::Entity>> {
            sqlx::query_as!(
                Post,
                r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post WHERE id=$1"#,
                id
            )
                .fetch_optional(pool)
                .await
        }

        async fn insert(&self, pool: &Pool<Postgres>) -> Result<()> {
            sqlx::query!(
                r#"INSERT INTO post (title, slug, content_md) VALUES ($1, $2, $3)"#,
                self.title,
                self.slug,
                self.content_md
            )
            .execute(pool)
            .await?;
            Ok(())
        }

        async fn update(&self, pool: &Pool<Postgres>) -> Result<()> {
            sqlx::query!(
                r#"UPDATE post SET title=$1, slug=$2, content_md=$3 WHERE id=$4"#,
                self.title,
                self.slug,
                self.content_md,
                self.id
            )
            .execute(pool)
            .await?;
            Ok(())
        }

        async fn delete(pool: &Pool<Postgres>, id: i64) -> Result<()> {
            sqlx::query!("DELETE FROM post WHERE id=$1", id)
                .execute(pool)
                .await?;
            Ok(())
        }
    }
}
