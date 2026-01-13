use crate::Route;
use dioxus::prelude::*;

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
pub fn SearchBar(placeholder: String, value: String, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        div { class: "relative w-full md:flex-1",
            span { class: "material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-text-dark/40 dark:text-gray-400",
                "search"
            }
            input {
                class: "w-full bg-white dark:bg-[#2a2a2a] border border-text-dark/10 dark:border-white/10 rounded-md h-12 pl-10 pr-4 text-text-dark dark:text-white placeholder:text-text-dark/40 dark:placeholder:text-gray-400 focus:ring-primary-light focus:border-primary-light transition-all",
                placeholder: "{placeholder}",
                r#type: "text",
                value: "{value}",
                oninput: move |e| oninput.call(e),
            }
        }
    }
}
