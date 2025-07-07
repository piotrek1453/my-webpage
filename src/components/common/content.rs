use leptos::prelude::*;

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! { <div class="container mx-auto text-base w-full">{children()}</div> }
}
