use crate::components::common::*;
use crate::components::pages::*;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/my-webpage.css" />

        // sets the document title
        <Title text="Welcome to my page!" />

        // content for this welcome page
        <bars::NavBar />

        <Router>
            <main>
                <Routes fallback=|| error_responses::PageNotFound()>
                    <Route path=StaticSegment("/") view=homepage::HomePage />
                    // TODO: implement views for remaining paths
                    <Route path=StaticSegment("/projects") view=projects::Projects />
                    <Route path=StaticSegment("/blog") view=blog::Blog />
                    <Route path=StaticSegment("/about") view=about::About />
                    <Route path=StaticSegment("/contact") view=contact::Contact />
                </Routes>
            </main>
        </Router>

        // content for this welcome page
        <footer::PageFooter />
    }
}
