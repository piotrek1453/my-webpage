use crate::components::common::title::Title;
use crate::models::post::Post;
use crate::utils::content_parser::parse_markdown;
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
            // TODO: this method for generating preview might cut words in half and I don't think it's very performant: improve
            async move { parse_markdown(content, Some(500)).await.unwrap() }
        },
    );
    view! {
        <article class="p-6 rounded-lg border shadow-sm transition-shadow hover:shadow-md prose lg:prose-xl">
            <h1>{post.title.clone()}</h1>
            <Suspense fallback=|| {
                view! { <p>"Parsing markdown..."</p> }
            }>
                {move || {
                    html_content
                        .get()
                        .map(|html| {
                            view! { <div class="mb-4 max-w-none prose" inner_html=html></div> }
                        })
                }}

            </Suspense>
        </article>
    }
}

#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    let pool = match use_context::<sqlx::Pool<sqlx::Postgres>>() {
        None => {
            return Err(ServerFnError::ServerError(
                "Error accessing database connection pool".to_string(),
            ));
        }
        Some(p) => p,
    };

    let posts = match sqlx::query_as!(
        Post,
        "SELECT id, title, slug, content_md, created_at, updated_at FROM post"
    )
    .fetch_all(&pool)
    .await
    {
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        Ok(p) => p,
    };

    Ok(posts)
}
