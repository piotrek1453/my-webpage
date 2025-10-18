use super::prelude::*;
use crate::models::post::Post;
#[cfg(feature = "ssr")]
use crate::server::database::db_context::DbContext;

#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    DbContext::get()?.post.get_all().await
}

#[server]
pub async fn get_blogpost_by_id(id: i64) -> Result<Option<Post>, ServerFnError> {
    DbContext::get()?.post.get_by_id(id).await
}

#[server]
pub async fn create_blogpost(post: Post) -> Result<(), ServerFnError> {
    DbContext::get()?.post.create(post).await
}

#[server]
pub async fn update_blogpost(post: Post) -> Result<(), ServerFnError> {
    DbContext::get()?.post.update(post).await
}

#[server]
pub async fn delete_blogpost(id: i64) -> Result<(), ServerFnError> {
    DbContext::get()?.post.delete(id).await
}
