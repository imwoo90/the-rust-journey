use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Container(children: Element, class: Option<String>) -> Element {
    let base_class =
        "layout-content-container flex flex-col w-full max-w-5xl mx-auto px-4 sm:px-6 lg:px-8";
    let combined_class = if let Some(extra) = class {
        format!("{} {}", base_class, extra)
    } else {
        base_class.to_string()
    };

    rsx! {
        div { class: "{combined_class}", {children} }
    }
}

#[component]
pub fn Section(children: Element, class: Option<String>) -> Element {
    let combined_class = if let Some(extra) = class {
        format!("flex flex-col gap-6 {}", extra)
    } else {
        "flex flex-col gap-6".to_string()
    };

    rsx! {
        section { class: "{combined_class}", {children} }
    }
}

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

#[component]
pub fn PrimaryButton(
    text: String,
    to: Option<Route>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let class = "flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-6 bg-primary-light text-text-dark text-base font-bold leading-normal tracking-[0.015em] hover:opacity-90 transition-all active:scale-95 shadow-md hover:shadow-lg";

    if let Some(route) = to.clone() {
        rsx! {
            Link { to: route, class: "{class}",
                span { class: "truncate", "{text}" }
            }
        }
    } else {
        rsx! {
            button {
                class: "{class}",
                onclick: move |e| {
                    if let Some(handler) = onclick {
                        handler.call(e);
                    }
                },
                span { class: "truncate", "{text}" }
            }
        }
    }
}

#[component]
pub fn Input(
    label: Option<String>,
    id: String,
    placeholder: String,
    r#type: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = r#type.unwrap_or_else(|| "text".to_string());
    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            if let Some(l) = label {
                label {
                    class: "text-sm font-medium text-text-dark/60 dark:text-[#D4D4D4] transition-colors",
                    r#for: "{id}",
                    "{l}"
                }
            }
            input {
                class: "w-full bg-white dark:bg-background-dark border border-text-dark/10 dark:border-white/20 rounded-md h-11 px-4 text-base text-text-dark dark:text-white placeholder:text-text-dark/30 dark:placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light transition-all",
                id: "{id}",
                placeholder: "{placeholder}",
                r#type: "{input_type}",
                oninput: move |e| {
                    if let Some(handler) = oninput {
                        handler.call(e);
                    }
                },
            }
        }
    }
}

#[component]
pub fn TextArea(
    label: Option<String>,
    id: String,
    placeholder: String,
    rows: Option<usize>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let rows_count = rows.unwrap_or(6);
    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            if let Some(l) = label {
                label {
                    class: "text-sm font-medium text-text-dark/60 dark:text-[#D4D4D4] transition-colors",
                    r#for: "{id}",
                    "{l}"
                }
            }
            textarea {
                class: "w-full bg-white dark:bg-background-dark border border-text-dark/10 dark:border-white/20 rounded-md p-4 text-base text-text-dark dark:text-white placeholder:text-text-dark/30 dark:placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light transition-all",
                id: "{id}",
                placeholder: "{placeholder}",
                rows: "{rows_count}",
                oninput: move |e| {
                    if let Some(handler) = oninput {
                        handler.call(e);
                    }
                },
            }
        }
    }
}

#[component]
pub fn SectionTitle(title: String) -> Element {
    rsx! {
        h2 { class: "text-text-dark dark:text-white text-3xl font-bold leading-tight tracking-[-0.015em] border-b border-text-dark/10 dark:border-white/10 pb-3 transition-colors",
            "{title}"
        }
    }
}

#[component]
pub fn TimelineItem(
    date: String,
    title: String,
    description: String,
    is_last: Option<bool>,
) -> Element {
    let spacing_class = if is_last.unwrap_or(false) {
        ""
    } else {
        "mb-10"
    };
    rsx! {
        div { class: "{spacing_class} ml-4 relative",
            div { class: "absolute w-4 h-4 bg-primary rounded-full mt-1.5 -left-6.5 border border-background-light dark:border-background-dark" }
            time { class: "mb-1 text-sm font-normal leading-none text-text-dark/40 dark:text-gray-400 transition-colors",
                "{date}"
            }
            h3 { class: "text-lg font-semibold text-text-dark dark:text-white transition-colors",
                "{title}"
            }
            p { class: "text-base font-normal text-text-dark/70 dark:text-[#D4D4D4] transition-colors mt-1",
                "{description}"
            }
        }
    }
}
