# The Rust Journey Agent Guide & Standards (AGENTS.md)

This file defines the project-specific coding guidelines, dynamic content architecture, and verification standards for The Rust Journey blog.

---

## 1. Architectural Baseline: Agent-Native Core
This project strictly adheres to the compiler-enforced **Agent-Native** architecture.

👉 **Mandatory Baseline Reading**: Consult [AGENT_NATIVE.md](file://AGENT_NATIVE.md) for the authoritative specification on:
- **Core Philosophy**: Living LLM-Wiki (`mod.rs` = `index.md`), compile-checked intra-doc links, and living doctests.
- **Compiler Hard Limits**: Rules 1-4 enforced via `build.rs` and `build_linter.rs` (file headers, logical code limits, inline test limits, function physical limits).
- **Anti-Code-Golfing Principle**: Decompose into cohesive submodules; never compress variable names or eliminate idiomatic whitespace.
- **Safety Lints**: `missing_docs = "deny"`, `clippy::unwrap_used = "deny"`, `clippy::expect_used = "deny"` in `Cargo.toml`.

---

## 2. Blog Architecture & Dynamic Content Specifics

This project uses a custom architecture for dynamic Markdown content, static asset resolution, and deployment on GitHub Pages. Follow these rules strictly:

### 1. Dynamic Path Resolution (Critical)
To support GitHub Pages subpaths (e.g., `/my_blog/`) without hardcoding paths:
- **Rule**: Use `crate::data::utils::get_base_path()` for ALL resource fetching and relative asset paths.
- **Why**: It reads the `<base href="...">` tag injected during CI. Prepend it to any absolute-like paths.

```rust
// ✅ CORRECT: Prepend get_base_path()
let url = format!("{}/content/posts/{}/index.md", get_base_path(), id);
let img_src = format!("{}/{}", get_base_path(), post.image_url);

// ❌ INCORRECT (Will fail on GitHub Pages)
let url = format!("content/posts/{}/index.md", id);
```

### 2. Dynamic Content & Indexing
- **Storage**: Markdown content is stored in `public/content/{type}/{id}/index.md`.
- **Fetching**: Content is NOT bundled. Use `gloo-net` to fetch `.md` files at runtime.
- **Automation**: `build.rs` automatically scans content and generates `posts_index.json` / `projects_index.json`. 
- **Image Co-location**: Place images in the same folder as the `index.md`. Reference them by filename; `build.rs` will resolve them.

### 3. Component & SEO Patterns
- **SEO Title**: Use `document::Title { "{page_name} - Rust's Horizon" }` in every view for dynamic browser tab titles.
- **Markdown Rendering**: Use `crate::data::utils::markdown_to_html(content, id, type)` for professional rendering with syntax highlighting and image resolution.
- **Shared Layouts**: Use `EntryHero` as the standard header for both blog and project post views.

### 4. Async Resource Handling
Always handle the three states of a `use_resource`:
1. `None`: Return a loading spinner/skeleton.
2. `Some(None)`: Return a 404/Not Found UI.
3. `Some(Some(data))`: Render the actual content.

---

## 3. Verification Protocol
Always verify code changes before completing any task:

```bash
# 1. Check compiler linter rules (Agent-Native constraints)
cargo check

# 2. Run unit tests and integration tests
cargo test --all-targets

# 3. Verify Clippy lints and clean idioms
cargo clippy --all-targets

# 4. Verify doc link integrity and eliminate broken references
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# 5. Verify browser runtime behavior via Playwright E2E interactive suite
npm run test:e2e
```

---

## 4. AI Agent Navigation Guide
1. **Entry Points**: Start with `AGENTS.md` (this file) and `README.md` to grasp project architecture and verification requirements.
2. **Module Indexing**: Treat `mod.rs` in any directory as the module's architecture map. Read the `//!` header to understand submodules and dependencies before diving into child files.
3. **Graph Traversal via Rustdoc Links**: Follow compile-checked intra-doc links (`[Type]`) to traverse dependencies deterministically.
4. **Pre-commit Verification**: Never consider a task complete without passing all 5 steps in the Verification Protocol.

---

# Appendix: Dioxus 0.7 Reference Guide

You are working with [0.7 Dioxus](https://dioxuslabs.com/learn/0.7). Dioxus 0.7 changes every API in Dioxus. `cx`, `Scope`, and `use_state` are gone.

### UI with RSX
```rust
rsx! {
    div {
        class: "container",
        color: "red",
        width: if condition { "100%" },
        "Hello, Dioxus!"
    }
    for i in 0..5 {
        div { "{i}" }
    }
    if condition {
        div { "Condition is true!" }
    }
}
```

### Components
* Components are functions annotated with `#[component]`.
* Names must start with a capital letter or contain an underscore.
* Props must be owned (`String`, `Vec<T>`), and implement `PartialEq` and `Clone`. Wrap in `ReadOnlySignal` for reactive Copy props.

### State & Signals
* `use_signal` creates local component state.
* Call `my_signal()` to read / clone.
* Use `*my_signal.write() = ...` or `my_signal.with_mut(...)` to mutate.
* `use_memo` recalculates when read signals change.
* `use_context_provider` and `use_context` for subtree dependency injection.

### Async & Web
* In WASM client code, avoid server-only macros. Use standard Dioxus hooks, `spawn(...)`, and async resources.
