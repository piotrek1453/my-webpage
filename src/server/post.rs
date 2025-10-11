use super::prelude::*;
use crate::models::post::Post;

#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    sqlx::query_as!(
        Post,
        "SELECT id, title, slug, content_md, created_at, updated_at FROM post"
    )
    .fetch_all(&use_context::<sqlx::Pool<sqlx::Postgres>>().ok_or_else(|| {
        ServerFnError::<std::convert::Infallible>::ServerError(
            "Error accessing database connection pool".into(),
        )
    })?)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database query failed: {e}")))
}
