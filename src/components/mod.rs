//! # Shared Component Architecture Module
//!
//! Organizes high-level interactive widgets, galleries, comment integrations, and social share links.
//! Re-exports UI primitives from the `ui` submodule to streamline view assembly.

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
