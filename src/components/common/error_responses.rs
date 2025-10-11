use leptos::prelude::*;

#[component]
pub fn PageNotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use axum::http::StatusCode;
        use leptos_axum::ResponseOptions;
        if let Some(res) = leptos::prelude::use_context::<ResponseOptions>() {
            res.set_status(StatusCode::NOT_FOUND);
        }
    }
    view! { <h1>Page not found</h1> }
}
