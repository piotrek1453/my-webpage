use crate::server::prelude::*;
use std::sync::OnceLock;

/// Global singleton for DbContext - initialized once and reused across all requests.
static DB_CONTEXT: OnceLock<Pool<Postgres>> = OnceLock::new();

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
    ///
    /// This is more efficient than repeatedly calling use_context in every server function.
    pub fn get() -> Result<Self, ServerFnError> {
        let pool = DB_CONTEXT.get_or_init(|| {
            use_context::<Pool<Postgres>>().expect("DB pool must be available in Leptos context")
        });
        Ok(DbContext {
            pool,
            post: PostRepository { pool },
        })
    }

    /// Get reference to the underlying pool for custom queries if needed
    pub fn pool(&self) -> &Pool<Postgres> {
        self.pool
    }
}

/// Namespace for Post table operations
use post::Post;
pub struct PostRepository {
    pool: &'static Pool<Postgres>,
}

impl PostRepository {
    pub async fn get_all(&self) -> Result<Vec<post::Post>, ServerFnError> {
        sqlx::query_as!(
            Post,
            r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post"#
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Failed to fetch all posts: {e}")))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<post::Post>, ServerFnError> {
        sqlx::query_as!(
            Post,
            r#"SELECT id, title, slug, content_md, created_at, updated_at FROM post WHERE id=$1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Failed to fetch post by id: {id}: {e}")))
    }

    pub async fn create(&self, post: post::Post) -> Result<(), ServerFnError> {
        sqlx::query!(
            r#"INSERT INTO post (title, slug, content_md) VALUES ($1, $2, $3)"#,
            post.title,
            post.slug,
            post.content_md
        )
        .execute(self.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Failed to create post: {e}")))
        .map(|_| ())
    }

    pub async fn update(&self, post: post::Post) -> Result<(), ServerFnError> {
        sqlx::query!(
            r#"UPDATE post SET title=$1, slug=$2, content_md=$3 WHERE id=$4"#,
            post.title,
            post.slug,
            post.content_md,
            post.id
        )
        .execute(self.pool)
        .await
        .map_err(|e| {
            ServerFnError::ServerError(format!("Failed to update post with id {0}: {e}", post.id))
        })
        .map(|_| ())
    }

    pub async fn delete(&self, id: i64) -> Result<(), ServerFnError> {
        sqlx::query!("DELETE FROM post WHERE id=$1", id)
            .execute(self.pool)
            .await
            .map_err(|e| {
                ServerFnError::ServerError(format!("Failed to delete post with id {id}: {e}"))
            })
            .map(|_| ())
    }
}
