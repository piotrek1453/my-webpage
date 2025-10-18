pub use crate::models::post::Post;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

/// Global singleton for Postgres connection pool - initialized once and reused across all requests.
static DB_CONNECTION_POOL: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

/// Universal database context for all tables/models.
/// Provides a single access point to the database with repository pattern
/// implemented for each of the corresponding {struct: table} pair.
///
/// # Example
/// ```rust
/// let ctx = DbContext::get()?;
/// let posts = ctx.post.get_all().await?;
/// let post = ctx.post.get_by_id(1).await?;
/// ctx.post.create(new_post).await?;
/// ```
pub struct DbContext {
    /// Direct pool access for custom queries
    pool: &'static Pool<Postgres>,
    /// Post table operations
    pub post: PostRepository,
}

impl DbContext {
    /// Get the global DbContext singleton.
    /// First call initializes it from Leptos context, subsequent calls reuse the same instance.
    pub async fn get() -> Result<Self, sqlx::Error> {
        if let Some(pool) = DB_CONNECTION_POOL.get() {
            return Ok(DbContext {
                pool,
                post: PostRepository { pool },
            });
        }

        let database_url =
            dotenvy::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

        let new_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&database_url)?;

        // Try to install the pool; ignore error if another thread won the race.
        let _ = DB_CONNECTION_POOL.set(new_pool);

        // Either we just set it or another thread set it concurrently.
        let pool = match DB_CONNECTION_POOL.get() {
            Some(p) => p,
            None => {
                let err = std::io::Error::other("DB_CONNECTION_POOL not initialized");
                return Err(sqlx::Error::Configuration(Box::new(err)));
            }
        };

        Ok(DbContext {
            pool,
            post: PostRepository { pool },
        })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        tracing::info!("Running database migrations");
        match sqlx::migrate!("./migrations").run(self.pool).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!(error = %e, "Failed running migrations");
                Err(e)
            }
        }
    }

    // commented out for now - probably to be removed to avoid side-access to db
    // /// Get reference to the underlying pool for custom queries if needed
    // pub fn pool(&self) -> &Pool<Postgres> {
    //     self.pool
    // }
}

/// Namespace for Post table operations
pub struct PostRepository {
    pool: &'static Pool<Postgres>,
}

impl PostRepository {
    pub async fn get_all(&self) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post"#
        )
        .fetch_all(self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post WHERE id=$1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
    }

    pub async fn create(&self, post: Post) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO post (title, slug, content_md) VALUES ($1, $2, $3)"#,
            post.title,
            post.slug,
            post.content_md
        )
        .execute(self.pool)
        .await
        .map(|_| ())
    }

    pub async fn update(&self, post: Post) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE post SET title=$1, slug=$2, content_md=$3 WHERE id=$4"#,
            post.title,
            post.slug,
            post.content_md,
            post.id
        )
        .execute(self.pool)
        .await
        .map(|_| ())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM post WHERE id=$1", id)
            .execute(self.pool)
            .await
            .map(|_| ())
    }
}
