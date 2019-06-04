//! Thin wrapper for Zephyr bindings.
//!
//! Encapsulates `unsafe` codes into this crate and provides `safe` interface
//! for application.

#![no_std]

#[macro_use]
pub mod io;
