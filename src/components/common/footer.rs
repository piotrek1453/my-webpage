use leptos::prelude::*;

#[component]
pub fn PageFooter() -> impl IntoView {
    #[cfg(feature = "ssr")]
    let current_year = {
        use time::OffsetDateTime;
        OffsetDateTime::now_utc().year().to_string()
    };

    #[cfg(feature = "hydrate")]
    let current_year = String::new();

    view! {
        <footer id="page-footer" class="text-center mt-4 text-sm text-gray-500">
            <p class="flex items-center justify-center gap-2">
                {format!("© {current_year} Piotr Jucha")}
                <a
                    href="https://github.com/piotrek1453/my-webpage"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-block"
                >
                    <img
                        src="/static/github-mark.svg"
                        alt="GitHub logo (light mode)"
                        class="block dark:hidden w-8 h-8"
                    />
                    <img
                        src="/static/github-mark-white.svg"
                        alt="GitHub logo (dark mode)"
                        class="hidden dark:block w-8 h-8"
                    />
                </a>- licensed under BSD 3-Clause license.
            </p>
        </footer>
    }
}
