use crate::components::common::{content::Content, title::Title};
use crate::utils::content_parser;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title title="About" />
        <Content>
            {content_parser::parse_markdown(
                content_parser::read_markdown_from_file("static/test.md".to_string()),
            )}
        </Content>
    }
}
