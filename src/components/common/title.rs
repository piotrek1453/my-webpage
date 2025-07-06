use leptos::prelude::*;

#[component]
pub fn Title(title: &'static str) -> impl IntoView {
    view! {
        <div id="about" class="container gap-6 grid grid-cols-1 pt-8 mx-auto text-left">
            <h1 class="text-6xl tracking-tight mb-0.5">{title}</h1>
            <div class="divider w-36 my-0.5"></div>
        </div>
    }
}
