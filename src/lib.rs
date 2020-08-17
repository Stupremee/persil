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
//! // Your results will be stored in `./trace/{app-name}-{pid}`
//! persil::init("my_application");
//!
//! // To store the results in a custom path, use the `init_with_path` function.
//! persil::init_with_path("./");
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
use once_cell::sync::OnceCell;
use profiler::{Profiler, Sink, TimingGuard};

pub(crate) static PROFILER: OnceCell<Profiler> = OnceCell::new();

type Guard = TimingGuard<'static, Sink>;

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
pub fn trace(category: impl AsRef<str>, event: impl AsRef<str>) -> Guard {
    PROFILER
        .get()
        .expect(
            "persil profiler is not initialized. hint: initialize the profiler with `persil::init`",
        )
        .trace(category.as_ref(), event.as_ref())
}

/// Initializes the global profiler with the given application name.
///
/// The trace results will be stored in the `./trace` folder
/// and all files will have `name-<pid>` as their prefix.
///
/// # Panics
///
/// Panics if the profiler is already initialized.
pub fn init(name: impl AsRef<str>) {
    try_init(name).expect("persil profiler already initialized");
}

/// Tries to initialize global profiler with the given name.
///
/// The trace results will be stored in the `./trace` folder
/// and all files will have `name-<pid>` as their prefix.
///
/// # Return
///
/// Returns `Ok` if the profiler was successfully initialized
/// and `Err` if the profiler is already initialized.
pub fn try_init(name: impl AsRef<str>) -> Result<(), ()> {
    let path = format!("./trace/{}-{}", name.as_ref(), std::process::id());
    PROFILER.set(Profiler::new(path.as_ref())).map_err(|_| ())
}

/// Initializes the global profiler and will store the results
/// at the given path.
///
/// The results will be stored as `<path>.events`, `<path>.strings`, etc.
///
/// # Panics
///
/// Panics if the profiler is already initialized.
pub fn init_with_path(path: impl AsRef<std::path::Path>) {
    PROFILER
        .set(Profiler::new(path.as_ref()))
        .ok()
        .expect("persil profiler already initialized")
}

/// Tries to initializethe global profiler and will store the results
/// at the given path.
///
/// The results will be stored as `<path>.events`, `<path>.strings`, etc.
///
/// # Return
///
/// Returns `Ok` if the profiler was successfully initialized or `Err`
/// if the profiler is already initialized.
pub fn try_init_with_path(path: impl AsRef<std::path::Path>) -> Result<(), ()> {
    PROFILER.set(Profiler::new(path.as_ref())).map_err(|_| ())
}
