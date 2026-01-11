use dioxus::prelude::*;

#[component]
pub fn CallToAction() -> Element {
    rsx! {
        section { class: "border-t border-text-dark/10 dark:border-white/10 pt-12",
            div { class: "flex flex-col items-center text-center",
                h2 { class: "text-text-dark dark:text-white text-2xl font-bold", "Have a similar challenge?" }
                p { class: "text-text-dark/60 dark:text-gray-400 mt-2 max-w-xl",
                    "If you're looking to leverage Rust for high-performance web applications, embedded systems, or anything in between, let's talk."
                }
                a {
                    class: "inline-flex items-center justify-center gap-2 mt-6 bg-primary-light text-text-dark font-bold text-sm px-6 py-3 rounded-md hover:opacity-90 transition-all shadow-md active:scale-95",
                    href: "#",
                    "Get in Touch"
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            }
        }
    }
}
