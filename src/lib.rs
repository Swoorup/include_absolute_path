#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// The `include_absolute_path` macro returns the absolute path of a file or a directory.
///
/// This macro accepts both relative and absolute paths. If the path is relative,
/// it is considered relative to the file where the macro is called.
///
/// # Usage
///
/// ```rust
/// use include_absolute_path::include_absolute_path;
///
/// const FILE: &'static str = include_absolute_path!("../tests/test_file.txt");
/// ```
///
/// This will set `FILE` to the absolute path of the `src/main.rs` file relative to the file where the macro is called.
///
/// # Panics
///
/// This macro will panic if the specified file does not exist.
///
/// # Examples
///
/// ```compile_fail
/// use include_absolute_path::include_absolute_path;
///
/// const FILE: &'static str = include_absolute_path!("src/main.rs");
/// assert!(FILE.ends_with("src/main.rs"));
/// ```
///
/// ```rust
/// use include_absolute_path::include_absolute_path;
///
/// const FILE: &'static str = include_absolute_path!("/etc/passwd");
/// assert!(FILE.ends_with("/etc/passwd"));
/// ```
#[proc_macro]
pub fn include_absolute_path(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a string
    let path = parse_macro_input!(input as LitStr).value();
    let caller_file = proc_macro::Span::call_site().source_file().path();

    // Convert the path to a Path
    let path = std::path::Path::new(&path);

    // Check if the path is absolute
    let raw_path = if path.is_absolute() {
        // If the path is absolute, use it as is
        path.to_path_buf()
    } else {
        caller_file
            .parent()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get parent of the caller path: {}",
                    caller_file.display()
                )
            })
            .join(path)
    };

    let absolute_path = raw_path
        .canonicalize()
        .unwrap_or_else(|_| panic!("Failed to canonicalize path: {}", raw_path.display()));

    // Convert the path to a string
    let absolute_path_str = absolute_path
        .to_str()
        .unwrap_or_else(|| panic!("Failed to absolutize path: {}", raw_path.display()));

    // Return the absolute path as a string literal
    let expanded = quote! {
        #absolute_path_str
    };

    TokenStream::from(expanded)
}
