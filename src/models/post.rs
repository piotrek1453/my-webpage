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
