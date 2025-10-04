use crate::components::common::title::Title;
use crate::models::post::Post;
use crate::utils::content_parser::parse_markdown;
use anyhow::Result;
use leptos::prelude::*;

#[component]
pub fn Blog() -> impl IntoView {
    // Create a resource that fetches posts from the server
    let posts = Resource::new(|| (), |_| async { get_blogposts().await });

    view! {
        <Title title="Blog" />
        <div class="container py-8 px-4 mx-auto">
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
                                        <div class="space-y-6">
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

// TODO: add displaying dates
#[component]
pub fn BlogPostPreview(post: Post) -> impl IntoView {
    let content_md = post.content_md.clone();
    let html_content = Resource::new(
        || (),
        move |_| {
            let content = content_md.clone();
            async move { parse_markdown(content).await.unwrap() }
        },
    );
    view! {
        <article class="overflow-hidden relative py-4 px-10 w-full max-w-full h-96 border border-black opacity-80 transition-all dark:border-white hover:shadow-lg hover:opacity-100 shadow-xs shadow-gray-400 rounded-4xl prose lg:prose-xl">

            <div class="py-4 px-2 w-full max-w-full prose lg:prose-xl">
                <h6 class="font-bold text-cyan-400 dark:text-orange-400">#{post.id}</h6>
                <h1>{post.title.clone()}</h1>
                <Suspense fallback=|| {
                    view! { <p>"Parsing markdown..."</p> }
                }>
                    {move || {
                        html_content
                            .get()
                            .map(|html| {
                                view! {
                                    <div class="mb-4 w-full max-w-full prose" inner_html=html></div>
                                }
                            })
                    }}
                </Suspense>
            </div>

            <div class="absolute bottom-0 left-0 w-full h-36 bg-gradient-to-t from-white to-transparent pointer-events-none dark:from-gray-900"></div>
        </article>
    }
}

#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    sqlx::query_as!(
        Post,
        "SELECT id, title, slug, content_md, created_at, updated_at FROM post"
    )
    .fetch_all(&use_context::<sqlx::Pool<sqlx::Postgres>>().ok_or_else(|| {
        ServerFnError::<std::convert::Infallible>::ServerError(
            "Error accessing database connection pool".into(),
        )
    })?)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database query failed: {e}")))
}
