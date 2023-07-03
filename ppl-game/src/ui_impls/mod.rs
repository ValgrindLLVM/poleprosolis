//! # User Interface Implementations
//!
//! 1. [`tui`] terminal user interface. Implemented for unix(-like) os. Requires feature `tui`

#[cfg(feature = "tui")]
pub mod tui;
