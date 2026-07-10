use crate::components::{
    Comments, ContentGallery, DetailHero, GalleryItem, RouteFactory, ShareButtons,
};
use crate::data::blog::{derive_categories, fetch_all_posts, get_post_by_id, Post};
use crate::data::constants::APP_TITLE;
use crate::data::utils::markdown_to_html;
use crate::hooks::{use_syntax_highlighting, use_mermaid};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    let posts_res = use_server_future(fetch_all_posts)?;
    let posts_guard = posts_res.read();
    let posts = posts_guard.as_ref().unwrap();

    let blog_items = posts
        .iter()
        .map(|post| GalleryItem {
            id: post.id.clone(),
            title: post.title.clone(),
            description: post.description.clone(),
            image_url: post.image_url.clone(),
            tags: post.tags.clone(),
        })
        .collect();

    rsx! {
        ContentGallery {
            title: "The Journey's Log",
            subtitle: "Documenting every breakthrough and lesson learned while navigating the Rust ecosystem—from bare-metal firmware to cloud-native services.",
            search_placeholder: "Search articles...",
            items: blog_items,
            categories: derive_categories(posts),
            route_factory: RouteFactory(|id| Route::BlogPost { id }),
        }
    }
}

#[component]
pub fn BlogPost(id: String) -> Element {
    let mut current_id = use_signal(|| id.clone());
    if current_id() != id {
        current_id.set(id.clone());
    }

    let post_res = use_server_future(move || {
        let id = current_id();
        async move { get_post_by_id(&id).await }
    })?;

    use_syntax_highlighting();
    use_mermaid();

    let posts_guard = post_res.read();
    let post_opt = posts_guard.as_ref().unwrap();

    match post_opt {
        Some(post) => {
            let html_content = markdown_to_html(&post.content, &post.meta.id, "posts");
            let img_url = if post.meta.image_url.is_empty() {
                "".to_string()
            } else if post.meta.image_url.starts_with("http") {
                post.meta.image_url.clone()
            } else if post.meta.image_url.starts_with('/') {
                format!("https://imwoo90.github.io/the-rust-journey{}", post.meta.image_url)
            } else {
                format!("https://imwoo90.github.io/the-rust-journey/content/posts/{}/{}", post.meta.id, post.meta.image_url)
            };

            rsx! {
                document::Title { "{post.meta.title} - {APP_TITLE}" }
                document::Meta { name: "description", content: post.meta.description.clone() }
                document::Meta { name: "keywords", content: post.meta.tags.join(", ") }
                document::Meta { name: "author", content: post.meta.author.clone() }
                
                // Open Graph / Facebook
                document::Meta { property: "og:title", content: post.meta.title.clone() }
                document::Meta { property: "og:description", content: post.meta.description.clone() }
                document::Meta { property: "og:type", content: "article" }
                document::Meta { property: "og:url", content: format!("https://imwoo90.github.io/the-rust-journey/blog/{}", post.meta.id) }
                if !img_url.is_empty() {
                    document::Meta { property: "og:image", content: img_url.clone() }
                }
                
                // Twitter
                document::Meta { name: "twitter:card", content: "summary_large_image" }
                document::Meta { name: "twitter:title", content: post.meta.title.clone() }
                document::Meta { name: "twitter:description", content: post.meta.description.clone() }
                if !img_url.is_empty() {
                    document::Meta { name: "twitter:image", content: img_url.clone() }
                }

                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        DetailHero {
                            title: post.meta.title.clone(),
                            author: post.meta.author.clone(),
                            date: post.meta.date.clone(),
                            read_time: post.get_read_time(),
                            back_link: Route::BlogList {},
                            back_label: "Blog".to_string(),
                        }

                        div {
                            class: "prose max-w-none dark:prose-invert",
                            dangerous_inner_html: "{html_content}",
                        }

                        ShareButtons { title: post.meta.title.clone() }

                        SeriesNavigation { current_post: post.clone() }

                        Comments {}
                    }
                }
            }
        }
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Post Not Found" }
                Link {
                    to: Route::BlogList {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Blog"
                }
            }
        },
    }
}

#[component]
fn SeriesNavigation(current_post: Post) -> Element {
    let posts_res = use_server_future(fetch_all_posts)?;
    let posts_guard = posts_res.read();

    if let (Some(series_name), Some(posts)) =
        (current_post.meta.series.as_ref(), posts_guard.as_ref())
    {
        // 1. 같은 시리즈 글들만 필터링
        let mut series_posts: Vec<_> = posts
            .iter()
            .filter(|p| p.series.as_ref() == Some(series_name))
            .collect();

        // 2. series_order 기준으로 오름차순 정렬 (기본값 0)
        series_posts.sort_by_key(|p| p.series_order.unwrap_or(0));

        // 3. 현재 글의 위치(인덱스) 찾기
        if let Some(current_index) = series_posts
            .iter()
            .position(|p| p.id == current_post.meta.id)
        {
            // 연재물이 2개 이상일 때만 표시
            if series_posts.len() < 2 {
                return rsx! { "" };
            }

            let prev_post = if current_index > 0 {
                series_posts.get(current_index - 1)
            } else {
                None
            };

            let next_post = if current_index < series_posts.len() - 1 {
                series_posts.get(current_index + 1)
            } else {
                None
            };

            return rsx! {
                div { class: "mt-12 pt-8 border-t border-text-dark/10 dark:border-text-light/10",
                    div { class: "flex flex-col gap-4",
                        h3 { class: "text-sm font-semibold uppercase tracking-wider text-text-dark/50 dark:text-text-light/50",
                            "More from: {series_name}"
                        }
                        div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                            // Previous Card
                            if let Some(prev) = prev_post {
                                Link {
                                    to: Route::BlogPost { id: prev.id.clone() },
                                    class: "group flex flex-col p-4 rounded-xl border border-text-dark/10 dark:border-text-light/10 hover:border-primary-light transition-all duration-300",
                                    span { class: "text-xs text-text-dark/60 dark:text-text-light/60 mb-1 flex items-center gap-1",
                                        span { class: "material-symbols-outlined text-[14px]", "arrow_back" }
                                        "Previous"
                                    }
                                    span { class: "font-medium group-hover:text-primary-light transition-colors line-clamp-1",
                                        "{prev.title}"
                                    }
                                }
                            } else {
                                // 이전 글이 없으면 빈 공간 유지 (레이아웃 정렬용)
                                div { class: "hidden sm:block" }
                            }

                            // Next Card
                            if let Some(next) = next_post {
                                Link {
                                    to: Route::BlogPost { id: next.id.clone() },
                                    class: "group flex flex-col p-4 rounded-xl border border-text-dark/10 dark:border-text-light/10 hover:border-primary-light transition-all duration-300 items-end text-right",
                                    span { class: "text-xs text-text-dark/60 dark:text-text-light/60 mb-1 flex items-center gap-1",
                                        "Next"
                                        span { class: "material-symbols-outlined text-[14px]", "arrow_forward" }
                                    }
                                    span { class: "font-medium group-hover:text-primary-light transition-colors line-clamp-1",
                                        "{next.title}"
                                    }
                                }
                            } else {
                                div { class: "hidden sm:block" }
                            }
                        }
                    }
                }
            };
        }
    }

    rsx! { "" }
}
