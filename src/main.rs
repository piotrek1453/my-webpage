#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]
#![deny(unused_must_use)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use my_webpage::app::*;
    use my_webpage::server::database::db_context::DbContext;
    use tower_http::trace::TraceLayer;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load variables from .env before reading configuration / DATABASE_URL
    tracing::info!("Loading variables from .env file");
    let _ = dotenvy::dotenv().ok();

    // Load variables needed for Leptos
    tracing::info!("Loading Leptos config");
    let conf = match get_configuration(None) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "Failed to read Leptos configuration");
            std::process::exit(1);
        }
    };
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // DB setup
    tracing::info!("Setting up the database");
    let db_context = match DbContext::get().await {
        Ok(ctx) => ctx,
        Err(e) => {
            tracing::error!(error = %e, "Failed to initialize database context");
            std::process::exit(1);
        }
    };

    if let Err(e) = db_context.migrate().await {
        tracing::error!(error = %e, "Failed to run database migrations");
        std::process::exit(1);
    }
    tracing::info!("Database migrations completed successfully");

    // Generate the list of routes in Leptos app and pass stuff to context
    tracing::info!("Setting up routes");
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!(error = %e, addr = %addr, "Failed to bind TcpListener");
            std::process::exit(1);
        }
    };
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        tracing::error!(error = %e, "Axum server error");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
