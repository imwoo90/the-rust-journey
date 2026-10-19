//! # Data Access Layer Subsystem (index.md)
//!
//! ## Overview
//! Provides data structures, loaders, and indexing utilities for blog posts and project showcase entries.
//! Centralizes path resolution, markdown frontmatter parsing, and runtime static JSON ingestion.
//!
//! ## Submodules
//! - [`blog`] (`blog.rs`): Blog post data structures, metadata parsing, and list ingestion.
//! - [`projects`] (`projects.rs`): Showcase project entry models, link targets, and catalog loaders.
//! - [`constants`] (`constants.rs`): Site-wide global configuration keys, titles, and layout constants.
//! - [`utils`] (`utils.rs`): Markdown-to-HTML parser with syntax highlighting and base path resolution (`get_base_path`).
//!
//! ## Search Tags
//! #data, #blog-metadata, #project-metadata, #markdown-parser, #base-path

pub mod blog;
pub mod constants;
pub mod projects;
pub mod utils;
