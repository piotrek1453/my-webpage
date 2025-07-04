use leptos::prelude::*;

/// Documentation for [`NavBar`]
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
        <div class="navbar">
            // rounded-2xl shadow-sm bg-gray-100 dark:bg-gradient-to-br dark:
            // bg-linear-to-b dark:from-purple-600 dark:via-blue-500 dark:to-gray-600
            // dark:border-amber-600 border-1"
            <div class="navbar-start">
                <a class="btn btn-ghost text-xl">daisyUI</a>
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
        </div>
    }
}
