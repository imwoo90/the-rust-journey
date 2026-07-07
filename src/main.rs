// Main entry point for the Dioxus blog application
use dioxus::prelude::*;
use views::{About, BlogList, BlogPost, Contact, Home, Navbar, NotFound, ProjectList, ProjectPost};

mod components;
mod data;
mod hooks;
mod views;

use data::constants::FAVICON;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/blog")]
    BlogList {},

    #[route("/blog/:id")]
    BlogPost { id: String },

    #[route("/projects")]
    ProjectList {},

    #[route("/projects/:id")]
    ProjectPost { id: String },

    #[route("/about")]
    About {},

    #[route("/contact")]
    Contact {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const MAIN_CSS: Asset = asset!("assets/tailwind.css");

#[server(endpoint = "static_routes")]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    println!("static_routes called. Cwd: {:?}", std::env::current_dir());
    let mut routes = vec![
        "/".to_string(),
        "/blog".to_string(),
        "/projects".to_string(),
        "/about".to_string(),
        "/contact".to_string(),
    ];

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let posts_dir = manifest_dir.join("public/content/posts");
    let projects_dir = manifest_dir.join("public/content/projects");

    match std::fs::read_dir(&posts_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        if let Some(name) = entry.file_name().to_str() {
                            routes.push(format!("/blog/{}", name));
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read {:?}: {:?}", posts_dir, e);
        }
    }

    match std::fs::read_dir(&projects_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        if let Some(name) = entry.file_name().to_str() {
                            routes.push(format!("/projects/{}", name));
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read {:?}: {:?}", projects_dir, e);
        }
    }

    println!("static_routes returning: {:?}", routes);
    Ok(routes)
}

#[cfg(feature = "server")]
async fn strip_base_path(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    if path.starts_with("/the-rust-journey") {
        let new_path = &path["/the-rust-journey".len()..];
        let new_path = if new_path.is_empty() { "/" } else { new_path };
        let mut parts = uri.into_parts();
        let query = parts.path_and_query.as_ref().and_then(|pq| pq.query()).unwrap_or("");
        let new_pq = if query.is_empty() {
            new_path.to_string()
        } else {
            format!("{}?{}", new_path, query)
        };
        parts.path_and_query = Some(new_pq.parse().unwrap());
        *req.uri_mut() = axum::http::Uri::from_parts(parts).unwrap();
    }
    next.run(req).await
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let static_dir = std::env::var("DIOXUS_PUBLIC_PATH")
        .map(std::path::PathBuf::from)
        .ok()
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|parent| parent.join("public")))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("./public"));
    
    let incremental_cfg = dioxus::server::IncrementalRendererConfig::new()
        .static_dir(static_dir);

    let serve_cfg = dioxus::server::ServeConfig::builder()
        .incremental(incremental_cfg);

    let addr = dioxus_cli_config::fullstack_address_or_localhost();
    
    use dioxus::server::DioxusRouterExt;
    let router = axum::Router::new()
        .serve_dioxus_application(serve_cfg.clone(), App)
        .layer(axum::middleware::from_fn(strip_base_path));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

/// Detect initial theme (Pure Rust abstraction)
fn get_initial_theme() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(saved)) = storage.get_item("theme") {
                    return saved == "dark";
                }
            }
            if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
                return mql.matches();
            }
        }
    }
    // Default for Desktop/SSR
    false
}

/// Sync theme to storage and document root (Pure Rust abstraction)
#[warn(unused_variables)]
fn sync_theme(_is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // 1. Sync with document root (html tag) so that body background and Tailwind variants work correctly
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.class_list().toggle_with_force("dark", _is_dark);
                }
            }
            // 2. Persist to localStorage
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if _is_dark { "dark" } else { "light" });
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn App() -> Element {
    let is_dark = use_signal(get_initial_theme);
    use_context_provider(|| is_dark);

    // Platform-agnostic effect
    use_effect(move || sync_theme(is_dark()));

    rsx! {

        // Standard Links
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "true",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;700;900&family=Roboto+Mono&family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap",
            rel: "stylesheet",
        }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        // Root Wrapper: Reacts to is_dark signal
        div { class: if is_dark() { "dark" } else { "" },
            div { class: "bg-background-light dark:bg-background-dark text-text-dark dark:text-text-light min-h-screen transition-colors duration-300",
                Router::<Route> {}
            }
        }
    }
}
