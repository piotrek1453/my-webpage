pub mod database;

pub mod prelude {
    #[cfg(feature = "ssr")]
    pub use super::database::*;
    pub use leptos::prelude::*;
    #[cfg(feature = "ssr")]
    pub use sqlx::{Pool, Postgres};
    pub use std::convert::Infallible;
}

pub mod markdown;
pub mod post;

#[cfg(feature = "ssr")]
pub use database::db_context::DbContext;
