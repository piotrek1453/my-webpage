use leptos::prelude::*;

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! {
        <div class="container mx-auto w-full text-base prose prose-zinc md:prose-xl lg:prose-xl">
            {children()}
        </div>
    }
}
