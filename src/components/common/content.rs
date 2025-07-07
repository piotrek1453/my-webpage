use leptos::prelude::*;

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! { <div class="container mx-auto text-base">{children()}</div> }
}

#[component]
pub fn Image(image_path: &'static str) -> impl IntoView {
    view! {
        <div class="container mx-auto text-base">
            <img src=image_path alt="" />
        </div>
    }
}
