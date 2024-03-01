#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// The `include_file_path` macro returns the absolute path of a file.
///
/// This macro accepts both relative and absolute paths. If the path is relative,
/// it is considered relative to the file where the macro is called.
///
/// # Usage
///
/// ```rust
/// use include_file_path::include_file_path;
/// 
/// const FILE: &'static str = include_file_path!("../tests/test_file.txt");
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
/// use include_file_path::include_file_path;
/// 
/// const FILE: &'static str = include_file_path!("src/main.rs");
/// assert!(FILE.ends_with("src/main.rs"));
/// ```
///
/// ```rust
/// use include_file_path::include_file_path;
/// 
/// const FILE: &'static str = include_file_path!("/etc/passwd");
/// assert_eq!(FILE, "/etc/passwd");
/// ```
#[proc_macro]
pub fn include_file_path(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a string
    let path = parse_macro_input!(input as LitStr).value();
    let caller_file = proc_macro::Span::call_site().source_file().path();

    // Convert the path to a Path
    let path = std::path::Path::new(&path);

    // Check if the path is absolute
    let absolute_path = if path.is_absolute() {
        // If the path is absolute, use it as is
        path.to_path_buf()
    } else {
        let current_dir = caller_file.parent().unwrap();
        current_dir.join(path)
    };

    // Check if the file exists
    if !absolute_path.exists() {
        panic!("File does not exist: {}", absolute_path.display());
    }

    // Convert the path to a string
    let absolute_path_str = absolute_path.to_str().unwrap();

    // Return the absolute path as a string literal
    let expanded = quote! {
        #absolute_path_str
    };

    TokenStream::from(expanded)
}
