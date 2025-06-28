use chrono::Datelike;
use leptos::prelude::*;

#[component]
pub fn PageFooter() -> impl IntoView {
    let current_year = chrono::Utc::now().year();
    view! {
      <footer id="page-footer">
        <small>{format!("© {current_year} Piotr Jucha")}</small>
      </footer>
    }
}
