use crate::models::post::Post;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::server::database::db_context::DbContext;

#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    let ctx = DbContext::get()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ctx.post
        .get_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_blogpost_by_id(id: i64) -> Result<Option<Post>, ServerFnError> {
    let ctx = DbContext::get()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ctx.post
        .get_by_id(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn create_blogpost(post: Post) -> Result<(), ServerFnError> {
    let ctx = DbContext::get()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ctx.post
        .create(post)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn update_blogpost(post: Post) -> Result<(), ServerFnError> {
    let ctx = DbContext::get()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ctx.post
        .update(post)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn delete_blogpost(id: i64) -> Result<(), ServerFnError> {
    let ctx = DbContext::get()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ctx.post
        .delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
