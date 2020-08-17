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
//! // You have to call `init` at the start of the program,
//! // with the name of your application.
//! //
//! // Your results will be stored in `./trace/{app-name}`
//! persil::init("my_application");
//!
//! // To store the results in a custom path, use the `init_with_path` function.
//! persil::init_with_path("my_application", "./");
//!
//! // `trace` will start tracing an event.
//! // An event is composed of a `category` and a `label`.
//! // The `trace` function returns guard, that will stop tracing,
//! // if it's dropped.
//! {
//!   let _profiler = persil::trace("Parsing", "Expression");
//!   let expr = parse_expression().unwrap();
//!
//!   // `_profiler` is dropped here so it will stop tracing
//!   // at the end of this scope
//! }
//!
//! let profiler = persil::trace("Parsing", "Item");
//! parse_item().unwrap();
//!
//! // You can also drop the guard manually to stop tracing.
//! drop(profiler);
//!
//! # fn parse_expression() -> Option<Expr> { Some(Expr) }
//! # fn parse_item() -> Option<Item> { Some(Item) }
//! # struct Expr;
//! # struct Item;
//! ```
//!
//! **Enable the profiler in your binary.**
//!
//! To enable profiling, you have to enable the `profiler`
//! feature in this crate. Otherwise the `trace` function will do nothing.
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

#[cfg(feature = "profiler")]
mod profiler;
#[cfg(feature = "profiler")]
use {
    once_cell::sync::OnceCell,
    profiler::{Profiler, Sink, TimingGuard},
};

#[cfg(feature = "profiler")]
pub(crate) static PROFILER: OnceCell<Profiler> = OnceCell::new();

#[cfg(feature = "profiler")]
type Guard = TimingGuard<'static, Sink>;
#[cfg(not(feature = "profiler"))]
type Guard = ();

macro_rules! feature_impl {
    ($a:expr, $b:expr) => {
        #[cfg(feature = "profiler")]
        {
            $a
        }
        #[cfg(not(feature = "profiler"))]
        {
            $b
        }
    };
}

/// Starts tracing an event with the given `category` and a `name`.
///
/// This function will return a guard that will stop tracing if it gets dropped.
/// It will only trace events if the `profiler` build feature is enabled.
/// If the features is disabled, `trace` is just a no-op function.
///
/// # Example
///
/// ```
/// {
///     let _guard = persil::trace("Some", "Event");
///     some_time_intense_event();
/// }
///
/// fn some_time_intense_event() {
///     // ...
/// }
/// ```
#[allow(unused_variables)]
pub fn trace(category: impl AsRef<str>, event: impl AsRef<str>) -> Guard {
    feature_impl! {
        PROFILER
        .get()
        .expect(
            "persil profiler is not initialized. hint: initialize the profiler with `persil::init`",
        )
        .trace(category.as_ref(), event.as_ref()),
        ()
    }
}

/// Initializes the global profiler with the given application name.
///
/// The trace results will be stored in the `./trace` folder
/// and all files will have `name-<pid>` as their prefix.
#[allow(unused_variables)]
pub fn init(name: impl AsRef<str>) {
    feature_impl! {
        {
            let path = format!("./trace/{}-{}", name.as_ref(), std::process::id());
            PROFILER.set(Profiler::new(path.as_ref())).ok().expect("persil profiler already initialized");
        },
        ()
    }
}

/// Initializes the global profiler and will store the results
/// at the given path.
///
/// The results will be stored as `<path>.events`, `<path>.strings`, etc.
#[allow(unused_variables)]
pub fn init_with_path(path: impl AsRef<std::path::Path>) {
    feature_impl! {
        PROFILER.set(Profiler::new(path.as_ref())).ok().expect("persil profiler already initialized"),
        ()
    }
}
