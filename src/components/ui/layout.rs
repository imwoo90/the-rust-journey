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
