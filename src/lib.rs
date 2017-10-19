//! Low-level Rust bindings for [yoga](https://facebook.github.io/yoga)
//!
//! # Usage
//!
//! Building yoga-sys will build the yoga C library, so you don't need
//! to do anything else than include this library as part of your dependencies.
//!
//! It’s [on crates.io](https://crates.io/crates/libffi-sys), so you
//! can do:
//!
//! ```toml
//! [dependencies]
//! yoga-sys = "0.1.0"
//! ```
//!
//! to your `Cargo.toml` and
//!
//! ```rust
//! extern crate yoga_sys;
//! ```
//!
//! to your crate root.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

mod ffi;

pub use ffi::*;

#[cfg(test)]
mod tests;
