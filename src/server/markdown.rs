pub use leptos::prelude::*;

#[server]
pub async fn parse_markdown(md: String) -> Result<String, ServerFnError> {
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html_output = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);

    Ok(html_output)
}
