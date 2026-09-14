//! # Shared UI Primitives Submodule
//!
//! ## Overview
//! Exposes layout containers, hero headers, responsive cards, form controls, and badge elements.
//! Follows design tokens defined in Tailwind CSS for consistent dark and light mode themes.
//!
//! ## Submodules
//! - [`cards`]: Interactive content presentation cards with hover elevation and badges.
//! - [`display`]: Visual presentation elements including badges, icons, and status indicators.
//! - [`forms`]: Styled user input controls, text areas, and submission action buttons.
//! - [`hero`]: Page banner headers with headlines, subtitles, and call-to-action buttons.
//! - [`layout`]: Structural container primitives, responsive section wrappers, and grids.
//!
//! ## Search Tags
//! #ui, #design-system, #cards, #forms, #hero, #layout

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
