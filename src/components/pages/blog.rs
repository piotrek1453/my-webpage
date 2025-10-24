use crate::components::common::title::Title;
use crate::models::post::Post;
use crate::server::{markdown::parse_markdown, post::get_blogpost_by_slug, post::get_blogposts};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[cfg(feature = "ssr")]
static DATE_FORMAT: &[time::format_description::FormatItem] =
    time::macros::format_description!("[day]-[month]-[year]");

#[component]
pub fn Blog() -> impl IntoView {
    // Create a resource that fetches posts from the server
    let posts = Resource::new(|| (), |_| async { get_blogposts().await });

    view! {
        <Title title="Blog" />
        <div class="container py-2 px-2 mx-auto">
            <Suspense fallback=|| {
                view! { <p>"Loading posts..."</p> }
            }>
                {move || {
                    posts
                        .get()
                        .map(|result| match result {
                            Ok(posts_list) => {
                                if posts_list.is_empty() {
                                    view! { <p>"No posts found."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            {posts_list
                                                .into_iter()
                                                .map(|post| view! { <BlogPostPreview post=post /> })
                                                .rev()
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                            Err(_) => {
                                view! { <p class="text-red-500">"Error loading posts."</p> }
                                    .into_any()
                            }
                        })
                }}

            </Suspense>
        </div>
    }
}

#[component]
pub fn BlogPostPreview(post: Post) -> impl IntoView {
    let content_md = post.content_md.clone();
    let html_content = Resource::new(
        || (),
        move |_| {
            let content = content_md.clone();
            async move {
                parse_markdown(content)
                    .await
                    .unwrap_or_else(|_| "<p>Error parsing markdown</p>".to_string())
            }
        },
    );

    let (created_at, updated_at): (String, String) = {
        #[cfg(feature = "ssr")]
        {
            let fmt = |dt: Option<time::OffsetDateTime>| {
                dt.and_then(|d| d.format(&DATE_FORMAT).ok())
                    .unwrap_or_else(|| "–".to_string())
            };
            (fmt(post.created_at), fmt(post.updated_at))
        }
        #[cfg(not(feature = "ssr"))]
        {
            (String::new(), String::new())
        }
    };

    view! {
        <article class="overflow-hidden relative rounded-xl border shadow-md transition-all duration-300 hover:shadow-xl card bg-base-200 border-base-300 group dark:bg-base-300 dark:border-base-content/20">
            <div class="py-4 px-4 pb-2 space-y-1 card-body">

                <a href=format!("/{}", post.slug.trim_start_matches('/')) target="_blank">

                    // <!-- Header -->
                    <div class="flex justify-between items-center text-xs opacity-70">
                        <span class="font-mono text-cyan-500 dark:text-orange-400">#{post.id}</span>
                        <span>
                            <em>Created at: {created_at.clone()}</em>
                            " · "
                            <em>Updated at: {updated_at.clone()}</em>
                        </span>
                    </div>

                    // <!-- Title -->
                    <h2 class="text-base font-semibold leading-snug transition-colors card-title text-base-content group-hover:text-primary">
                        {post.title.clone()}
                    </h2>

                    // <!-- Markdown preview -->
                    <div class="relative">
                        <Suspense fallback=|| {
                            view! { <p class="text-sm opacity-60">"Parsing markdown..."</p> }
                        }>
                            {move || {
                                html_content
                                    .get()
                                    .map(|html| {
                                        view! {
                                            <div
                                                class="overflow-hidden relative max-w-none text-sm leading-relaxed whitespace-pre-wrap break-words prose prose-sm line-clamp-5 [&_h1]:m-0 [&_h2]:m-0 [&_h3]:m-0 [&_h4]:m-0 [&_h5]:m-0 [&_h6]:m-0 [&_p]:m-0 [&>*:first-child]:mt-0 [&>*:last-child]:mb-0 dark:prose-invert"
                                                inner_html=html
                                            ></div>
                                        }
                                    })
                            }}
                        </Suspense>

                        // <!-- Gradient overlay + button -->
                        <div class="absolute bottom-0 left-0 w-full h-16">
                            <div class="absolute inset-0 bg-gradient-to-t to-transparent pointer-events-none from-base-200 dark:from-base-300"></div>
                            <div class="flex relative z-10 justify-end pt-3 pr-2">
                                <button class="normal-case shadow-sm btn btn-xs btn-primary sm:btn-sm">
                                    Read more
                                </button>
                            </div>
                        </div>
                    </div>
                </a>

            </div>
        </article>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());
    let post = Resource::new(slug, |slug| async move {
        get_blogpost_by_slug(slug).await.ok().flatten()
    });
    view! {
        <Suspense fallback=move || {
            view! { <p>Loading...</p> }
        }>

            {move || match post.get() {
                Some(Some(post)) => {
                    view! {
                        <article>
                            <h1>{post.title}</h1>
                        </article>
                    }
                        .into_any()
                }
                Some(None) => {

                    view! { <p>Post not found</p> }
                        .into_any()
                }
                None => {

                    view! { <p>Loading...</p> }
                        .into_any()
                }
            }}
        </Suspense>
    }
}
