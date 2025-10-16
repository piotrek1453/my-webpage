#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use my_webpage::app::*;
    use sqlx::postgres::PgPoolOptions;
    use tower_http::trace::TraceLayer;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load variables from .env before reading configuration / DATABASE_URL
    tracing::info!("Loading variables from .env file");
    let _ = dotenvy::dotenv().ok();

    // Load variables needed for Leptos
    tracing::info!("Loading Leptos config");
    let conf = get_configuration(None).expect("Error reading Leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // DB setup
    tracing::info!("Setting up the database");
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error creating PgPool");

    // DB migrations
    tracing::info!("Running migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Error running migrations");

    // Generate the list of routes in Leptos app and pass stuff to context
    tracing::info!("Setting up routes");
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Error starting TcpListener");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Error starting Axum server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
