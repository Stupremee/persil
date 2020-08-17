# Persil

[![Docs.rs][docs-badge]][docs-link]
[![Crates.io][crate-badge]][crate-link]

[docs-badge]: https://docs.rs/persil/badge.svg
[docs-link]: https://docs.rs/persil
[crate-badge]: https://img.shields.io/crates/v/persil.svg
[crate-link]: https://crates.io/crates/persil

## Table of Contents
+ [About](#about)
+ [Getting Started](#getting_started)
+ [Usage](#usage)

## About <a name = "about"></a>

Persil is a minimal and simple library for profiling events.
It's based on rust's [measureme](https://docs.rs/measureme) and
is just a simple, but powerful layer ontop of `measureme`.

## Getting Started <a name = "getting_started"></a>

### Prerequisites

- Rust (I don't have any minimun required rust version, just try one of the latest)
- Tools for reading the serialized data. (See [here](https://github.com/rust-lang/measureme))
- Obviously an application that you want to profile.

### Installing

Add this to your `Cargo.toml`
```
[dependencies]
persil = "0.1.0"
```

If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed
```
cargo add persil
```

## Usage <a name = "usage"></a>

```rust
// You have to call `init` at the start of the program,
// with the name of your application.
//
// Your results will be stored in `./trace/{app-name}`
persil::init("my_application");

// To store the results in a custom path, use the `init_with_path` function.
persil::init_with_path("my_application", "./");

// `trace` will start tracing an event.
// An event is composed of a `category` and a `label`.
// The `trace` function returns guard, that will stop tracing,
// if it's dropped. 
{
  let _profiler = persil::trace("Parsing", "Expression");
  let expr = parse_expression().unwrap();

  // `_profiler` is dropped here so it will stop tracing
  // at the end of this scope
}

let profiler = persil::trace("Parsing", "Item");
parse_item().unwrap();

// You can also drop the guard manually to stop tracing.
drop(profiler);
```

**Enable the profiler in your binary.**

To enable profiling, you have to enable the `profiler`
feature in this crate. Otherwise the `trace` function will do nothing.

**Analyze the results**

To analye and display the results, you can use one of the tools in the [measureme repo](https://github.com/rust-lang/measureme).

For example to use `summarize`, just do:
```sh
# if you changed the path to the results, use the new path
summarize trace/my_application
```

For more information checkout the [measureme](https://github.com/rust-lang/measureme) repository.
