//! # Shared Component Architecture Module
//!
//! ## Overview
//! Organizes high-level interactive widgets, galleries, comment integrations, and social share links.
//! Re-exports UI primitives from the `ui` submodule to streamline view assembly.
//!
//! ## Submodules
//! - [`call_to_action`]: Call-to-action banners and conversion prompt blocks.
//! - [`comments`]: Utterances-based GitHub issue commenting integration widget.
//! - [`gallery`]: Paginated or categorized media and post card gallery grids.
//! - [`share_buttons`]: Social platform sharing triggers and link copy utilities.
//! - [`ui`]: Foundational design system primitives, cards, buttons, and layout containers.
//!
//! ## Search Tags
//! #components, #widgets, #gallery, #comments, #share-buttons

pub mod call_to_action;
pub mod comments;
pub mod gallery;
pub mod share_buttons;
pub mod ui;

#[allow(unused_imports)]
pub use call_to_action::*;
pub use comments::*;
pub use gallery::*;
pub use share_buttons::*;
pub use ui::*;
