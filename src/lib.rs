//! # rugger
//!
//! A simple and flexible logging crate for Rust.
//!
//! ## Quick Start
//!
//! ```rust
//! use rugger::{log_info, log_warning, log_error};
//!
//! log_info!("Application started");
//! log_warning!("Disk space low: {}%", 12);
//! log_error!("Connection failed");
//! ```
//!
//! ## Log Levels
//!
//! | Macro | Output |
//! |---|---|
//! | `log_info!` | stdout |
//! | `log_warning!` | stderr |
//! | `log_error!` | stderr |
//!
//! ## Tee (log + file)
//!
//! Use `tee_*!` macros to log to both stdout/stderr and a file:
//!
//! ```rust
//! use rugger::tee_info;
//!
//! tee_info!("/tmp/app.log", "Logged to console and file");
//! ```
//!
//! ## Custom Log Levels
//!
//! Implement `Display` and `LogLevel` on your own enum:
//!
//! ```rust,ignore
//! use std::fmt::Display;
//! use rugger::LogLevel;
//!
//! enum MyLevel { Debug, Trace }
//!
//! impl Display for MyLevel {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         match self {
//!             Self::Debug => write!(f, "[DEBUG]"),
//!             Self::Trace => write!(f, "[TRACE]"),
//!         }
//!     }
//! }
//!
//! impl LogLevel for MyLevel {}
//! ```

mod log;
mod macros;
pub use log::*;

#[cfg(test)]
mod tests;
