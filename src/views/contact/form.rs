//! # Contact Form Submodule
//!
//! Encapsulates input fields, validation logic, submission delays,
//! and interactive error states for the contact communication interface.

use crate::components::{Input, SectionTitle, TextArea};
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

#[component]
fn ContactErrorAlert(status: Signal<String>, error_msg: Signal<String>) -> Element {
    if status() == "error" {
        rsx! {
            div { class: "flex items-start gap-3 bg-red-500/10 dark:bg-red-500/20 text-red-600 dark:text-red-400 p-4 rounded-lg border border-red-500/20 text-sm font-medium animate-shake",
                span { class: "material-symbols-outlined text-[20px] select-none", "error" }
                span { "{error_msg}" }
            }
        }
    } else {
        rsx! { "" }
    }
}

#[component]
fn ContactSubmitButton(status: Signal<String>) -> Element {
    rsx! {
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
            ContactErrorAlert { status, error_msg }
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
            ContactSubmitButton { status }
        }
    }
}
