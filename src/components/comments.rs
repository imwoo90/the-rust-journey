use dioxus::prelude::*;

#[component]
pub fn Comments() -> Element {
    #[cfg(target_arch = "wasm32")]
    let is_dark = use_context::<Signal<bool>>();
    
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let is_dark_val = is_dark();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // 1. Remove existing content to prevent duplicates during route transitions
                    if let Some(existing_container) = document.get_element_by_id("giscus-container") {
                        existing_container.set_inner_html("");
                    }
                    
                    // 2. Create and append the Giscus script element
                    if let Ok(script) = document.create_element("script") {
                        let theme = if is_dark_val { "dark" } else { "light" };
                        let _ = script.set_attribute("src", "https://giscus.app/client.js");
                        let _ = script.set_attribute("data-repo", "imwoo90/the-rust-journey");
                        let _ = script.set_attribute("data-repo-id", "R_kgDOQ3rGbw");
                        let _ = script.set_attribute("data-category", "General");
                        let _ = script.set_attribute("data-mapping", "pathname");
                        let _ = script.set_attribute("data-strict", "0");
                        let _ = script.set_attribute("data-reactions-enabled", "1");
                        let _ = script.set_attribute("data-emit-metadata", "0");
                        let _ = script.set_attribute("data-input-position", "bottom");
                        let _ = script.set_attribute("data-theme", theme);
                        let _ = script.set_attribute("data-lang", "ko");
                        let _ = script.set_attribute("crossorigin", "anonymous");
                        let _ = script.set_attribute("async", "true");
                        
                        if let Some(container) = document.get_element_by_id("giscus-container") {
                            let _ = container.append_child(&script);
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "mt-16 border-t border-solid border-text-dark/10 dark:border-white/10 pt-10",
            h2 { class: "text-2xl font-bold text-text-dark dark:text-white mb-6",
                "Comments"
            }
            div { id: "giscus-container", class: "w-full min-h-[150px]" }
        }
    }
}
