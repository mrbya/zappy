//! Ecosystem-specific adapters for Zappy.
//!
//! This crate is reserved for behavior that knows about specific ecosystems
//! such as Git, Cargo, uv, CMake, Zephyr, npm, Tauri, or future recipe mode.
//! Generic filesystem, hook, and core rendering behavior should not live here.

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
