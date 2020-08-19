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
It's based on rust's [measureme](https://docs.rs/measureme) serialization format and
thus the results can be analysed using the tools in the [measureme repository](https://github.com/rust-lang/measureme).

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

See [examples](https://github.com/Stupremee/persil/blob/master/examples/simple.rs) for usage.

**Analyze the results**

To analye and display the results, you can use one of the tools in the [measureme repo](https://github.com/rust-lang/measureme).

For example to use `summarize`, just do:
```sh
# if you changed the path to the results, use the new path
summarize trace/my_application
```
