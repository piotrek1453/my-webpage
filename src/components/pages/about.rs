use crate::components::common::{content::Content, content::Image, title::Title};
// use crate::utils::content_parser;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title title="About" />
        <Content>
            <Image image_path="static/github-mark.svg" />
        </Content>
    }
}
