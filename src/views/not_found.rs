use crate::components::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Container {
            main { class: "flex-grow flex flex-col items-center justify-center px-4 py-16 md:py-24 w-full min-h-[60vh] transition-colors duration-300",
                div { class: "w-full max-w-2xl bg-surface-dark dark:bg-background-darker border border-surface-border dark:border-white/10 rounded-xl shadow-2xl overflow-hidden mb-12",
                    div { class: "bg-[#1a110c] dark:bg-[#0a0a0a] px-4 py-2 border-b border-surface-border dark:border-white/10 flex items-center gap-2",
                        div { class: "w-3 h-3 rounded-full bg-red-500" }
                        div { class: "w-3 h-3 rounded-full bg-yellow-500" }
                        div { class: "w-3 h-3 rounded-full bg-green-500" }
                        span { class: "ml-2 text-xs text-gray-400 dark:text-gray-500 font-mono",
                            "terminal — cargo run"
                        }
                    }
                    div { class: "p-6 md:p-10 font-mono text-sm md:text-base leading-relaxed text-gray-300",
                        div { class: "flex gap-3 mb-2",
                            span { class: "text-green-500 select-none", "➜" }
                            span { class: "text-blue-400", "~" }
                            span { class: "text-gray-300", "cargo run --bin router" }
                        }
                        div { class: "text-gray-500 mb-4 select-none",
                            "   Compiling router v0.1.0 (/usr/src/rustdev)"
                        }
                        div { class: "text-gray-500 mb-4 select-none",
                            "    Finished dev [unoptimized + debuginfo] target(s) in 0.42s"
                        }
                        div { class: "text-gray-500 mb-4 select-none",
                            "     Running `target/debug/router`"
                        }
                        div { class: "mt-6 text-red-500 font-bold",
                            "thread 'main' panicked at 'Page not found: 404', src/main.rs:42:5"
                        }
                        div { class: "text-gray-500 mt-1",
                            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
                        }
                        div { class: "animate-pulse mt-4 w-3 h-5 bg-primary-light inline-block align-middle" }
                    }
                }
                div { class: "text-center space-y-6 max-w-[600px]",
                    h1 { class: "text-4xl md:text-5xl font-bold text-text-dark dark:text-white tracking-tight transition-colors",
                        span { class: "text-primary-light", "panic!" }
                        "(\"Lost in memory\");"
                    }
                    p { class: "text-text-dark/60 dark:text-gray-400 text-lg leading-relaxed transition-colors",
                        "The signal was lost. The pointer you are trying to dereference is null. The page might have been moved, deleted, or never existed in this memory block."
                    }
                    div { class: "flex flex-col sm:flex-row items-center justify-center gap-4 pt-4",
                        PrimaryButton { text: "Return to Base", to: Route::Home {} }
                    }
                }
            }
        }
    }
}
