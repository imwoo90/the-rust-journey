use crate::components::{Container, Hero, Input, Section, SectionTitle, TextArea};
use crate::data::constants::{
    APP_TITLE, AUTHOR_EMAIL, AUTHOR_NAME, GITHUB_URL, GITHUB_USERNAME, LINKEDIN_URL,
};
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut message = use_signal(|| "".to_string());
    let mut status = use_signal(|| "idle".to_string()); // "idle" | "submitting" | "success" | "error"
    let mut error_msg = use_signal(|| "".to_string());

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();

        let n = name().trim().to_string();
        let e_val = email().trim().to_string();
        let m = message().trim().to_string();

        if n.len() < 2 {
            error_msg.set("Please enter a name with at least 2 characters.".to_string());
            status.set("error".to_string());
            return;
        }

        if !e_val.contains('@') || !e_val.contains('.') || e_val.len() < 5 {
            error_msg.set("Please enter a valid email address.".to_string());
            status.set("error".to_string());
            return;
        }

        if m.len() < 10 {
            error_msg.set("Message must be at least 10 characters long.".to_string());
            status.set("error".to_string());
            return;
        }

        status.set("submitting".to_string());
        error_msg.set("".to_string());

        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            {
                gloo_timers::future::TimeoutFuture::new(1500).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            }

            status.set("success".to_string());
            name.set("".to_string());
            email.set("".to_string());
            message.set("".to_string());
        });
    };

    rsx! {
        document::Title { "Contact - {APP_TITLE}" }
        Container {
            Hero {
                title: "Get In Touch",
                subtitle: "Have a project in mind, a question about an article, or just want to connect? I'm always open to discussing new opportunities and collaborating on exciting ideas.",
            }

            Section { class: "px-4 mb-20",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12",
                    div { class: "md:col-span-2 bg-white dark:bg-[#2a2a2a] p-8 rounded-lg border border-text-dark/5 dark:border-white/10 transition-colors shadow-sm",
                        if status() == "success" {
                            div { class: "flex flex-col items-center justify-center py-12 text-center animate-fade-in",
                                div { class: "flex items-center justify-center size-16 bg-green-500/10 text-green-500 rounded-full mb-6 scale-110 transition-transform animate-bounce",
                                    span { class: "material-symbols-outlined text-4xl", "check_circle" }
                                }
                                h3 { class: "text-2xl font-bold text-text-dark dark:text-white mb-2", "Message Sent Successfully!" }
                                p { class: "text-base text-text-dark/70 dark:text-[#D4D4D4] max-w-md mx-auto mb-8",
                                    "Thank you for reaching out! I appreciate you taking the time to write, and I will get back to you as soon as possible."
                                }
                                button {
                                    class: "flex cursor-pointer items-center justify-center rounded-lg h-12 px-6 bg-primary-light text-text-dark text-base font-bold hover:opacity-90 active:scale-95 transition-all shadow-md",
                                    onclick: move |_| status.set("idle".to_string()),
                                    "Send Another Message"
                                }
                            }
                        } else {
                            form {
                                class: "flex flex-col gap-6",
                                onsubmit: handle_submit,
                                prevent_default: "onsubmit",
                                SectionTitle { title: "Send a Message" }
                                
                                if status() == "error" {
                                    div { class: "flex items-start gap-3 bg-red-500/10 dark:bg-red-500/20 text-red-600 dark:text-red-400 p-4 rounded-lg border border-red-500/20 text-sm font-medium animate-shake",
                                        span { class: "material-symbols-outlined text-[20px] select-none", "error" }
                                        span { "{error_msg}" }
                                    }
                                }

                                div { class: "flex flex-col sm:flex-row gap-6",
                                    Input {
                                        label: Some("Your Name".to_string()),
                                        id: "name",
                                        placeholder: "John Doe",
                                        value: name(),
                                        oninput: move |e: FormEvent| name.set(e.value()),
                                    }
                                    Input {
                                        label: Some("Your Email".to_string()),
                                        id: "email",
                                        placeholder: "john.doe@email.com",
                                        r#type: "email",
                                        value: email(),
                                        oninput: move |e: FormEvent| email.set(e.value()),
                                    }
                                }
                                TextArea {
                                    label: Some("Message".to_string()),
                                    id: "message",
                                    placeholder: "I'd like to discuss...",
                                    rows: 6,
                                    value: message(),
                                    oninput: move |e: FormEvent| message.set(e.value()),
                                }
                                div { class: "flex justify-start",
                                    if status() == "submitting" {
                                        button {
                                            class: "flex min-w-[150px] items-center justify-center rounded-lg h-12 px-6 bg-primary-light/50 text-text-dark text-base font-bold cursor-not-allowed shadow-md",
                                            disabled: true,
                                            span { class: "animate-spin mr-2 h-5 w-5 border-2 border-text-dark border-t-transparent rounded-full" }
                                            "Sending..."
                                        }
                                    } else {
                                        button {
                                            class: "flex min-w-[150px] cursor-pointer items-center justify-center rounded-lg h-12 px-6 bg-primary-light text-text-dark text-base font-bold hover:opacity-90 active:scale-95 transition-all shadow-md hover:shadow-lg",
                                            r#type: "submit",
                                            "Submit Message"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-6",
                        SectionTitle { title: "Contact Information" }
                        div { class: "flex flex-col gap-4",
                            ContactInfoItem {
                                icon: "mail",
                                label: "Email",
                                value: "{AUTHOR_EMAIL}",
                                href: "mailto:{AUTHOR_EMAIL}",
                            }
                            ContactInfoItem {
                                icon: "link",
                                label: "GitHub",
                                value: "@{GITHUB_USERNAME}",
                                href: "{GITHUB_URL}",
                            }
                            ContactInfoItem {
                                icon: "group",
                                label: "LinkedIn",
                                value: "{AUTHOR_NAME}",
                                href: "{LINKEDIN_URL}",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactInfoItem(icon: String, label: String, value: String, href: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "flex items-center justify-center size-10 bg-text-dark/5 dark:bg-white/10 rounded-full text-primary-light transition-colors",
                span { class: "material-symbols-outlined", "{icon}" }
            }
            div { class: "flex flex-col",
                p { class: "text-sm text-text-dark/40 dark:text-gray-400 transition-colors",
                    "{label}"
                }
                a {
                    class: "text-base font-medium text-text-dark dark:text-white hover:text-primary-light transition-colors",
                    href: "{href}",
                    "{value}"
                }
            }
        }
    }
}
