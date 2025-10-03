use crate::components::common::{content::Content, title::Title};
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! { <Title title="About" /> }
}
