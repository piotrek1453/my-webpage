use leptos::prelude::*;

#[component]
pub fn PageFooter() -> impl IntoView {
    let current_year = {
        #[cfg(feature = "ssr")]
        {
            use time::OffsetDateTime;
            OffsetDateTime::now_utc().year().to_string()
        }
        #[cfg(not(feature = "ssr"))]
        {
            String::new()
        }
    };

    view! {
        <footer id="page-footer" class="mt-4 text-sm text-center text-gray-500">
            <p class="flex gap-2 justify-center items-center">
                {format!("© {current_year} Piotr Jucha")}
                <a
                    href="https://github.com/piotrek1453/my-webpage"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-block"
                >
                    <img
                        src="github-mark.svg"
                        alt="GitHub logo (light mode)"
                        class="block w-8 h-8 dark:hidden"
                    />
                    <img
                        src="github-mark-white.svg"
                        alt="GitHub logo (dark mode)"
                        class="hidden w-8 h-8 dark:block"
                    />
                </a>- licensed under BSD 3-Clause license.
            </p>
        </footer>
    }
}
