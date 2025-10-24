use crate::components::common::{bars, error_responses, footer};
use crate::components::pages::{about, blog, contact, homepage, projects};

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    ParamSegment, StaticSegment,
    components::{Route, Router, Routes},
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
                    <Route path=StaticSegment("") view=homepage::HomePage />
                    <Route path=StaticSegment("projects") view=projects::Projects />
                    <Route path=StaticSegment("blog") view=blog::Blog />
                    <Route
                        path=(StaticSegment("blog"), StaticSegment("post"), ParamSegment("slug"))
                        view=blog::BlogPost
                    />
                    <Route path=StaticSegment("about") view=about::About />
                    <Route path=StaticSegment("contact") view=contact::Contact />
                </Routes>
            </main>
        </Router>

        // content for this welcome page
        <footer::PageFooter />
    }
}
