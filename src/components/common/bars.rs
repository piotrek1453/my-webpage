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
        <div class="navbar dark:bg-indigo-950/60 bg-base-100/80 shadow-md backdrop-blur-sm rounded-b-lg sticky top-0 z-50">
            <div class="navbar-start">
                // Mobile dropdown
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
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
                        class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
                    >
                        {nav_items
                            .iter()
                            .map(|(label, href)| {
                                view! {
                                    <li>
                                        <a class="btn-ghost rounded" href=*href>
                                            {*label}
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
                <a class="btn btn-ghost text-xl" href="/">
                    daisyUI
                </a>
            </div>
            <div class="navbar-center hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    {nav_items
                        .iter()
                        .map(|(label, href)| {
                            view! {
                                <li>
                                    <a class="btn-ghost rounded" href=*href>
                                        {*label}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
            // TODO: move theme switch here
            <div class="navbar-end">
                <ThemeSwitcher />
            </div>
        </div>
    }
}
