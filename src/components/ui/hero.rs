use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Hero(
    title: String,
    subtitle: Option<String>,
    centered: Option<bool>,
    children: Element,
) -> Element {
    let is_centered = centered.unwrap_or(true);
    let align_class = if is_centered {
        "text-center items-center"
    } else {
        "text-left items-start"
    };

    rsx! {
        div { class: "w-full flex flex-col gap-6 {align_class} px-4 py-10 md:py-16 transition-colors duration-300",
            div { class: "flex flex-col gap-4",
                h1 { class: "text-text-dark dark:text-white text-4xl md:text-6xl font-black leading-tight tracking-[-0.033em]",
                    "{title}"
                }
                if let Some(sub) = subtitle {
                    p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-lg md:text-xl font-normal leading-normal max-w-3xl mx-auto",
                        "{sub}"
                    }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DetailHero(
    title: String,
    author: String,
    date: String,
    read_time: String,
    back_link: Route,
    back_label: String,
) -> Element {
    rsx! {
        header { class: "flex flex-col gap-6 border-b border-text-dark/5 dark:border-white/5 pb-10",
            div { class: "flex items-center gap-2 text-sm text-text-dark/60 dark:text-gray-500 font-medium",
                Link {
                    class: "hover:text-primary-light transition-colors",
                    to: back_link,
                    "{back_label}"
                }
                span { "›" }
                span { class: "truncate text-text-dark/40 dark:text-gray-400", "{title}" }
            }
            h1 { class: "text-text-dark dark:text-white text-4xl sm:text-5xl font-extrabold leading-tight tracking-tight",
                "{title}"
            }
            div { class: "flex items-center gap-4 text-sm text-gray-400",
                span { class: "font-medium text-primary-light", "{author}" }
                span { class: "text-gray-600", "•" }
                time { "{date}" }
                span { class: "text-gray-600", "•" }
                span { "{read_time}" }
            }
        }
    }
}
