use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Card(
    title: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
    link_to: Option<Route>,
    external_link: Option<String>,
    link_text: Option<String>,
) -> Element {
    let link_label = link_text.unwrap_or_else(|| "Read More".to_string());

    let content = rsx! {
        div {
            class: "w-full aspect-video bg-cover bg-center rounded-t-lg",
            style: "background-image: url('{image_url}')",
        }
        div { class: "p-6 flex flex-col flex-grow",
            h3 { class: "text-text-dark dark:text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                "{title}"
            }
            p { class: "text-text-dark/70 dark:text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 flex-grow",
                "{description}"
            }
            div { class: "flex flex-wrap items-center gap-2 mb-4",
                for tag in tags {
                    Badge { text: tag }
                }
            }
            if link_to.is_some() {
                div { class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                    "{link_label} "
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            } else if let Some(url) = external_link {
                a {
                    class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                    href: "{url}",
                    "{link_label} "
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            }
        }
    };

    if let Some(target) = link_to.clone() {
        rsx! {
            Link {
                to: target,
                class: "flex flex-col rounded-lg overflow-hidden bg-white dark:bg-[#2a2a2a] border border-text-dark/5 dark:border-white/10 group transition-all duration-300 hover:shadow-lg dark:hover:bg-white/5",
                {content}
            }
        }
    } else {
        rsx! {
            div { class: "flex flex-col rounded-lg overflow-hidden bg-white dark:bg-[#2a2a2a] border border-text-dark/5 dark:border-white/10 group transition-all duration-300 hover:shadow-lg dark:hover:bg-white/5",
                {content}
            }
        }
    }
}

#[component]
pub fn Badge(text: String) -> Element {
    rsx! {
        span { class: "text-xs font-mono bg-text-dark/5 dark:bg-gray-700/50 text-text-dark/70 dark:text-gray-300 px-2 py-1 rounded transition-colors",
            "{text}"
        }
    }
}
