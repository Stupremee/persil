//! # Persil
//!
//! ## About
//!
//! Persil is a minimal and simple library for profiling events.
//! It's based on rust's [measureme](https://docs.rs/measureme) and
//! is just a simple, but powerful layer ontop of `measureme`.
//!
//! ## Getting Started
//!
//! ### Prerequisites
//!
//! - Rust (I don't have any minimun required rust version, just try one of the latest)
//! - Tools for reading the serialized data. (See [here](https://github.com/rust-lang/measureme))
//! - Obviously an application that you want to profile.
//!
//! ### Installing
//!
//! Add this to your `Cargo.toml`
//! ```ignore
//! [dependencies]
//! persil = "0.1.0"
//! ```
//!
//! If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed
//! ```ignore
//! cargo add persil
//! ```
//!
//! ## Usage
//!
//! ```
//! // todo
//! ```
//!
//! **Analyze the results**
//!
//! To analye and display the results, you can use one of the tools in the [measureme repo](https://github.com/rust-lang/measureme).
//!
//! For example to use `summarize`, just do:
//! ```sh
//! # if you changed the path to the results, use the new path
//! summarize trace/my_application
//! ```
//!
//! For more information checkout the [measureme](https://github.com/rust-lang/measureme) repository.
#![deny(rust_2018_idioms)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

mod profiler;

pub use profiler::*;
