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
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <div class="container gap-6 grid grid-cols-1 pt-20 mx-auto text-center">

            <h1 class="text-6xl tracking-wide">"Welcome to Leptos"</h1>

            <div class="mx-auto">
                <label class="flex cursor-pointer gap-2">
                    <span class="label-text">Current</span>
                    <input type="checkbox" value="cupcake" class="toggle theme-controller" />
                    <span class="label-text">Cupcake</span>
                </label>
            </div>

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
