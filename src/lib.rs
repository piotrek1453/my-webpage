pub mod app;
pub mod components;
pub mod models;
pub mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused_imports)]
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
