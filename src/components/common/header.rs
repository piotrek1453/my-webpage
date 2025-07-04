use leptos::prelude::*;
use leptos::web_sys;

#[component]
pub fn PageHeader() -> impl IntoView {
    // true = dark, false = light
    let is_dark = RwSignal::new(true);

    // Effect: update data-theme on <html> when toggled
    Effect::new(move |_| {
        let theme = if is_dark.get() { "night" } else { "cupcake" };
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                html.set_attribute("data-theme", theme).ok();
            }
        }
    });

    view! {
        <header class="flex items-center justify-between text-6x1 tracking-wide">
            <div>
                <h1>Welcome to my page!!!</h1>
            </div>
            <div>
                <label class="flex cursor-pointer gap-2 items-center">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="5" />
                        <path d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4" />
                    </svg>
                    <input
                        type="checkbox"
                        class="toggle theme-controller"
                        checked=is_dark.get()
                        on:input=move |ev| {
                            let checked = event_target_checked(&ev);
                            is_dark.set(checked);
                        }
                    />
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </label>
            </div>
        </header>
    }
}

// Helper to get checked state from event
fn event_target_checked(ev: &leptos::ev::Event) -> bool {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::HtmlInputElement;
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or(false)
}
