use crate::components::common::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let nav_items = [
        ("Home", "/"),
        ("Projects", "/projects"),
        ("Blog", "/blog"),
        ("About me", "/about"),
        ("Contact", "/contact"),
    ];

    view! {
        <div class="sticky top-0 z-50 rounded-b-lg shadow-md navbar bg-base-100/80 backdrop-blur-sm dark:bg-indigo-950/60">
            <div class="navbar-start">
                // Mobile dropdown
                <div class="dropdown">
                    <div tabindex="0" role="button" class="lg:hidden btn btn-ghost">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="w-5 h-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h8m-8 6h16"
                            />
                        </svg>
                    </div>
                    <ul
                        tabindex="0"
                        class="p-2 mt-3 w-52 shadow menu menu-sm dropdown-content bg-base-100 rounded-box z-1"
                    >
                        {nav_items
                            .iter()
                            .map(|(label, href)| {
                                view! {
                                    <li>
                                        <a class="rounded btn-ghost" href=*href>
                                            {*label}
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
                <a class="text-xl btn btn-ghost" href="/">
                    CDreamz
                </a>
            </div>
            <div class="hidden lg:flex navbar-center">
                <ul class="px-1 menu menu-horizontal">
                    {nav_items
                        .iter()
                        .map(|(label, href)| {
                            view! {
                                <li>
                                    <a class="rounded btn-ghost" href=*href>
                                        {*label}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
            <div class="navbar-end">
                <ThemeSwitcher />
            </div>
        </div>
    }
}
