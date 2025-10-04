pub mod app;
pub mod components;
pub mod models;
pub mod utils;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused_imports)]
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
