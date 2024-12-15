# `include_absolute_path` macro library
[![Latest Version](https://img.shields.io/crates/v/include_absolute_path.svg)](https://crates.io/crates/include_absolute_path) [![docs.rs](https://docs.rs/include_absolute_path/badge.svg)](https://docs.rs/include_absolute_path)  

This Rust project provides a procedural macro `include_absolute_path!` that returns the absolute path of a file or directory. 
The macro accepts both relative and absolute paths, and it will panic if the specified file does not exist.
While `include_str` exists, this can be useful for build/test scripts or for hot reloading files knowing that the relative path wouldn't change.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
include_absolute_path = { path = "0.1" }
```

Then, in your Rust code, you can use the include_absolute_path! macro as follows:

```rust
const FILE: &'static str = include_absolute_path!("src/main.rs");
```

This will set `FILE` to the absolute path of the `src/main.rs` file relative to the file where the macro is called.

## Environment Variable Support

The `include_absolute_path!` macro also supports environment variables. You can use environment variables in the path, and they will be expanded before the path is resolved.

```rust
const HOME_DIR: &'static str = include_absolute_path!("$HOME");
```

This will set `HOME_DIR` to the absolute path of the home directory.

## How It Works

The `include_absolute_path!` macro works by parsing the input path and checking if it's absolute. If it is, it returns the path as is. If it's not, it concatenates the path with the directory of the file where the macro is called to get the absolute path.

The macro uses the `proc_macro::Span::call_site().source_file().path()` function to get the path of the file where the macro is called. Currently this function is available as of this writing only on nightly via [`proc_macro_span`](https://github.com/rust-lang/rust/issues/54725) feature

Before returning the path, the macro checks if the file exists. If the file does not exist, it panics with a message indicating the file does not exist.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
