use crate::components::common::{bars::NavBar, footer::PageFooter, header::PageHeader};
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
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
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
        <Stylesheet id="leptos" href="/pkg/my-webpage.css"/>

        // sets the document title
        <Title text="Welcome to my page!"/>

        // content for this welcome page
        <PageHeader/>
        <NavBar/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    // TODO: implement views for remaining paths
                    <Route path=StaticSegment("/projects") view=Projects/>
                    <Route path=StaticSegment("/blog") view=Blog/>
                    <Route path=StaticSegment("/about") view=About/>
                    <Route path=StaticSegment("/contact") view=Contact/>
                </Routes>
            </main>
        </Router>

        <PageFooter/>
    }
}

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="fixed top-0 bottom-0 left-0 right-0 grid">{children()}</div> }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <div class="container gap-6 grid grid-cols-1 pt-20 mx-auto text-center">

            <h1 class="text-6xl tracking-wide">"Welcome to my page!"</h1>

            <div>
                <button
                    class="btn btn-primary my-4"
                    on:click=move |_| *count.write() += 1
                >
                    Click me
                    {count}
                </button>

            </div>
            <div>
                <button class="btn" onclick="my_modal_1.showModal()">
                    open modal
                </button>
                <dialog id="my_modal_1" class="modal">
                    <div class="modal-box">
                        <h3 class="text-lg font-bold">Hello!</h3>
                        <p class="py-4">Press ESC key or click the button below to close</p>
                        <div class="modal-action">
                            <form method="dialog">
                                <button class="btn">Close</button>
                            </form>
                        </div>
                    </div>
                </dialog>
            </div>
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <h1>Projects</h1>
    }
}

#[component]
fn Blog() -> impl IntoView {
    view! {
        <h1>Blog</h1>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h1>About</h1>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <h1>Contact</h1>
    }
}
