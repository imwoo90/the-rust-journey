//! # Contact Page View
//!
//! ## Overview
//! Provides the primary contact page view composing the interactive validation form,
//! message transmission status transitions, and external communication coordinates.
//!
//! ## Search Tags
//! #contact, #form-validation, #contact-info, #user-feedback

use crate::components::{Container, Hero, Input, Section, SectionTitle, TextArea};
use crate::data::constants::{APP_TITLE, AUTHOR_EMAIL, GITHUB_URL, GITHUB_USERNAME};
use dioxus::prelude::*;

/// Validates form inputs against minimum constraints before submission.
pub fn validate_contact_inputs(name: &str, email: &str, message: &str) -> Result<(), &'static str> {
    if name.len() < 2 {
        return Err("Please enter a name with at least 2 characters.");
    }
    if !email.contains('@') || !email.contains('.') || email.len() < 5 {
        return Err("Please enter a valid email address.");
    }
    if message.len() < 10 {
        return Err("Message must be at least 10 characters long.");
    }
    Ok(())
}

/// Simulates asynchronous message submission with timer delay.
pub fn trigger_submission(
    mut status: Signal<String>,
    mut name: Signal<String>,
    mut email: Signal<String>,
    mut message: Signal<String>,
) {
    status.set("submitting".to_string());
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
        name.set(String::new());
        email.set(String::new());
        message.set(String::new());
    });
}

/// Individual link item within the contact information sidebar.
#[component]
pub fn ContactInfoItem(icon: String, label: String, value: String, href: String) -> Element {
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

/// Renders author contact channels and social media profiles.
#[component]
pub fn ContactSidebarInfo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            SectionTitle { title: "Contact Information" }
            div { class: "flex flex-col gap-4",
                ContactInfoItem {
                    icon: "mail".to_string(),
                    label: "Email".to_string(),
                    value: AUTHOR_EMAIL.to_string(),
                    href: format!("mailto:{}", AUTHOR_EMAIL),
                }
                ContactInfoItem {
                    icon: "link".to_string(),
                    label: "GitHub".to_string(),
                    value: format!("@{}", GITHUB_USERNAME),
                    href: GITHUB_URL.to_string(),
                }
            }
        }
    }
}

/// Renders the confirmation state when a message is successfully delivered.
#[component]
fn ContactSuccessView(on_reset: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center py-12 text-center animate-fade-in",
            div { class: "flex items-center justify-center size-16 bg-green-500/10 text-green-500 rounded-full mb-6 scale-110 transition-transform animate-bounce",
                span { class: "material-symbols-outlined text-4xl", "check_circle" }
            }
            h3 { class: "text-2xl font-bold text-text-dark dark:text-white mb-2", "Message Sent Successfully!" }
            p { class: "text-base text-text-dark/70 dark:text-[#D4D4D4] max-w-md mx-auto mb-8",
                "Thank you for reaching out! I appreciate you taking the time to write, and I will get back to you as soon as possible."
            }
            button {
                class: "flex cursor-pointer items-center justify-center rounded-lg h-12 px-6 bg-primary text-white text-base font-semibold hover:bg-primary-hover active:scale-95 transition-all shadow-md",
                onclick: move |_| on_reset.call(()),
                "Send Another Message"
            }
        }
    }
}

/// Renders the contact input form with interactive validation indicators.
#[component]
pub fn ContactFormView(
    mut name: Signal<String>,
    mut email: Signal<String>,
    mut message: Signal<String>,
    status: Signal<String>,
    error_msg: Signal<String>,
    on_submit: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        form {
            class: "flex flex-col gap-6",
            onsubmit: move |e| on_submit.call(e),
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
                        class: "flex min-w-[150px] items-center justify-center rounded-lg h-12 px-6 bg-primary/50 text-white/80 text-base font-semibold cursor-not-allowed shadow-md",
                        disabled: true,
                        span { class: "animate-spin mr-2 h-5 w-5 border-2 border-white border-t-transparent rounded-full" }
                        "Sending..."
                    }
                } else {
                    button {
                        class: "flex min-w-[150px] cursor-pointer items-center justify-center rounded-lg h-12 px-6 bg-primary text-white text-base font-semibold hover:bg-primary-hover active:scale-95 transition-all shadow-md hover:shadow-lg",
                        r#type: "submit",
                        "Submit Message"
                    }
                }
            }
        }
    }
}

/// Renders the Contact page with interactive form validation and author information.
#[component]
pub fn Contact() -> Element {
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let message = use_signal(String::new);
    let mut status = use_signal(|| "idle".to_string());
    let mut error_msg = use_signal(String::new);

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();
        match validate_contact_inputs(name().trim(), email().trim(), message().trim()) {
            Ok(()) => {
                error_msg.set(String::new());
                trigger_submission(status, name, email, message);
            }
            Err(err) => {
                error_msg.set(err.to_string());
                status.set("error".to_string());
            }
        }
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
                    div { class: "md:col-span-2 bg-white dark:bg-surface-dark p-8 rounded-lg border border-text-dark/5 dark:border-white/10 transition-colors shadow-sm",
                        if status() == "success" {
                            ContactSuccessView {
                                on_reset: move |_| {
                                    let mut s = status;
                                    s.set("idle".to_string());
                                },
                            }
                        } else {
                            ContactFormView {
                                name,
                                email,
                                message,
                                status,
                                error_msg,
                                on_submit: handle_submit,
                            }
                        }
                    }
                    ContactSidebarInfo {}
                }
            }
        }
    }
}
