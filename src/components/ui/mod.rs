//! # Shared UI Primitives Submodule
//!
//! Exposes layout containers, hero headers, responsive cards, form controls, and badge elements.
//! Follows design tokens defined in Tailwind CSS for consistent dark and light mode themes.

pub mod cards;
pub mod display;
pub mod forms;
pub mod hero;
pub mod layout;

pub use cards::*;
pub use display::*;
pub use forms::*;
pub use hero::*;
pub use layout::*;
