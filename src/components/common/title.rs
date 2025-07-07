use leptos::prelude::*;

#[component]
pub fn Title(title: &'static str) -> impl IntoView {
    view! {
        <div class="container grid grid-cols-1 pt-8 mx-auto text-left">
            <h1 class="text-4xl tracking-tight">{title}</h1>
            <div class="divider w-36"></div>
        </div>
    }
}
