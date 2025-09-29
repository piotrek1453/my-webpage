#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use my_webpage::app::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use tower_http::{services::ServeDir, trace::TraceLayer};

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // Load variables from .env before reading configuration / DATABASE_URL
    tracing::info!("Loading variables from .env file");
    let _ = dotenvy::dotenv();

    tracing::info!("Loading config");
    // TODO: look into the config: there's something wrong with parsing the address from .env
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // DB setup
    tracing::info!("Setting up the database");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.unwrap();

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await.unwrap();
    tracing::info!("Test query result: {}", row.0);

    // Generate the list of routes in your Leptos App
    tracing::info!("Setting up routes");
    let routes = generate_route_list(App);
    let app = Router::new().nest_service("/public", ServeDir::new("public"))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options).layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
