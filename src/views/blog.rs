use crate::components::{
    BlogCategories, BlogSearch, Card, Comment, Comments, Container, EntryHero, Hero, Section,
    ShareButtons,
};
use crate::data::blog::{get_all_categories, get_all_posts, get_post_by_id};
use crate::data::utils::{get_base_path, markdown_to_html};
use crate::hooks::use_syntax_highlighting;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    let posts = get_all_posts();

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "From the Horizon",
                    subtitle: "Exploring the frontiers of Rust, from bare-metal to the web. Find articles, tutorials, and deep dives here.",
                }

                // Filter / Search Section
                section { class: "flex flex-col md:flex-row gap-4 px-4 items-center",
                    BlogSearch { placeholder: "Search articles..." }
                    BlogCategories {
                        categories: get_all_categories(),
                        active: "All".to_string(),
                    }
                }

                Section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for post in posts {
                        Card {
                            title: post.title.clone(),
                            description: post.description.clone(),
                            image_url: format!("{}/{}", get_base_path(), post.image_url),
                            tags: post.tags.clone(),
                            link_to: Route::BlogPost {
                                id: post.id.clone(),
                            },
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn BlogPost(id: String) -> Element {
    let post_resource = use_resource(move || {
        let id = id.clone();
        async move { get_post_by_id(&id).await }
    });

    use_syntax_highlighting();

    let resource = post_resource.read();
    match &*resource {
        Some(Some(post)) => {
            let html_content = markdown_to_html(&post.content, &post.meta.id, "posts");
            rsx! {
                document::Title { "{post.meta.title} - Rust's Horizon" }
                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        EntryHero {
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

                        ShareButtons {}

                        Comments {
                            comments: vec![
                                Comment {
                                    author: "Jane Cooper".to_string(),
                                    date: "2 days ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuATXU3JICms3OyNaZoVJNBpQGSqHbZXsY_slYkES2kANBTg2xB5sbEPC3DKz8hzBeoWeltrZouk9CE0hhlOFTusx7U27zIwssg-p4QLxBnxA_OSsCENLJL4PCYznC96ITW0l2xQeE6GXJ9_W47v_SekUzEdYhCdt-QanimUOBY9OtFM4a3imFZ5MMdRhY-tPvVhM2MSKpiVcDNWpCExdlaNFRyP4Sa-KFvTXyibaDzjq9ZhrXVuSRqFPOUT4Zv22es5AZFco4KpaBj7"
                                        .to_string(),
                                    text: "Great article! I was just looking for a good starting point for Rust on embedded. The code snippet is super helpful."
                                        .to_string(),
                                },
                                Comment {
                                    author: "Robert Fox".to_string(),
                                    date: "1 day ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuDucCURr32vwk41SdPofChh366sYzCkTMbv54gXm4lSWHQJdtJ2D341xf2qNZsIPM5oRjhBiULSxUfw2niGWlvHJOc72JArrLVdeylR7_QYduC2mvhBPwvZQoab83iys3HTJ_QBaWFWteNyXnCdmugcXK4PhVmq02ZLeD1ikjZJyJ4HoxNi7ene8vpXwM7yT3OE_C1JFe4hMA5t1hRdhJ2bxMTiy8Q1M6tT1fxQOTtW_-7XXKfNaywMDBRLmi8NxOTLzh0c05zLCzba"
                                        .to_string(),
                                    text: "Thanks for this! Could you do a follow-up on setting up a debugger with VS Code and probe-rs?"
                                        .to_string(),
                                },
                            ],
                        }
                    }
                }
                Footer {}
            }
        }
        Some(None) => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Post Not Found" }
                Link {
                    to: Route::BlogList {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Blog"
                }
            }
            Footer {}
        },
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-light" }
                p { class: "mt-4 text-text-dark/60 dark:text-text-light/60", "Loading article..." }
            }
            Footer {}
        },
    }
}
