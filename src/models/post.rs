use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content_md: String,

    // backend
    #[cfg(feature = "ssr")]
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,

    #[cfg(feature = "ssr")]
    #[serde(with = "time::serde::iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,

    // frontend
    #[cfg(not(feature = "ssr"))]
    pub created_at: Option<String>,

    #[cfg(not(feature = "ssr"))]
    pub updated_at: Option<String>,
}

#[cfg(feature = "ssr")]
pub mod db {
    use super::super::repository::Repository;
    use super::Post;
    use async_trait::async_trait;
    use sqlx::{Pool, Postgres, Result};

    #[async_trait]
    impl Repository for Post {
        type Entity = Post;

        async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Self::Entity>> {
            sqlx::query_as!(Post, r#"SELECT * FROM post"#)
                .fetch_all(pool)
                .await
        }

        async fn get_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Option<Self::Entity>> {
            sqlx::query_as!(Post, r#"SELECT * FROM post WHERE id=$1"#, id)
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
