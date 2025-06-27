extern crate proc_macro;

use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, LitStr, parse_macro_input};

/// Returns the absolute path of a file or directory at compile time.
///
/// This macro resolves both relative and absolute paths. Relative paths are resolved
/// relative to the file where the macro is called. The macro also supports environment
/// variable expansion using `$VAR` or `${VAR}` syntax.
///
/// # Features
///
/// - Resolves relative paths from the calling file's location
/// - Expands environment variables (e.g., `$HOME`, `${USER}`)
/// - Validates paths exist at compile time
/// - Provides detailed error messages with context
/// - Prevents path traversal attacks
///
/// # Usage
///
/// ```rust,ignore
/// use include_absolute_path::include_absolute_path;
///
/// // Relative path from current file
/// const CONFIG: &str = include_absolute_path!("../config.toml");
///
/// // Absolute path
/// const HOSTS: &str = include_absolute_path!("/etc/hosts");
///
/// // With environment variable
/// const HOME_DIR: &str = include_absolute_path!("$HOME");
/// const USER_CONFIG: &str = include_absolute_path!("$HOME/.config/app.conf");
/// ```
///
/// # Panics
///
/// This macro will panic at compile time if:
/// - The specified file or directory does not exist
/// - The path contains invalid UTF-8 characters
/// - Environment variable expansion fails
/// - The path contains suspicious traversal patterns (security check)
///
/// # Security
///
/// The macro includes basic security checks to prevent excessive path traversal
/// attempts. Paths with more than 3 `..` segments or where more than half the
/// components are `..` will be rejected.
///
/// # Examples
///
/// ## Relative path resolution
/// ```rust,ignore
/// use include_absolute_path::include_absolute_path;
///
/// // If called from src/main.rs, resolves to absolute path of Cargo.toml
/// const MANIFEST: &str = include_absolute_path!("../Cargo.toml");
/// ```
///
/// ## Environment variable expansion
/// ```rust
/// use include_absolute_path::include_absolute_path;
///
/// // Expands $HOME to user's home directory
/// const HOME: &str = include_absolute_path!("$HOME");
/// ```
///
/// ## Compile-time validation
/// ```compile_fail
/// use include_absolute_path::include_absolute_path;
///
/// // This will fail at compile time if the file doesn't exist
/// const MISSING: &str = include_absolute_path!("non_existent_file.txt");
/// ```
///
/// ## Security validation
/// ```compile_fail
/// use include_absolute_path::include_absolute_path;
///
/// // This will fail due to excessive path traversal
/// const SUSPICIOUS: &str = include_absolute_path!("../../../../../../../../etc/passwd");
/// ```
#[proc_macro]
pub fn include_absolute_path(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a string
    let lit_str = parse_macro_input!(input as LitStr);
    let path = lit_str.value();
    let span = lit_str.span();

    // Get the file path where macro is called
    let caller_file_str = proc_macro::Span::call_site()
        .local_file()
        .unwrap_or_else(|| {
            panic!(
                "Failed to get the source file location. \
            This should not happen on stable Rust."
            )
        });

    let caller_file = Path::new(&caller_file_str);

    // Expand environment variables in the path
    let expanded_path = match shellexpand::env(&path) {
        Ok(expanded) => expanded,
        Err(e) => panic!(
            "Failed to expand environment variable in path '{path}': {e}. \
            Make sure the environment variable exists and is valid."
        ),
    };

    // Convert the expanded path to a Path
    let path_buf = PathBuf::from(expanded_path.as_ref());

    // Validate for suspicious path patterns
    if contains_suspicious_patterns(&path_buf) {
        return Error::new(
            span,
            format!(
                "Path '{path}' contains suspicious traversal patterns. \
                For security reasons, paths with excessive '..' segments are not allowed."
            ),
        )
        .to_compile_error()
        .into();
    }

    // Check if the path is absolute
    let raw_path = if path_buf.is_absolute() {
        // If the path is absolute, use it as is
        path_buf
    } else {
        // Get parent directory of the caller file
        let parent = caller_file.parent().unwrap_or_else(|| {
            panic!(
                "Failed to get parent directory of the source file '{}'. \
                The file appears to be in the root directory.",
                caller_file.display()
            )
        });
        parent.join(&path_buf)
    };

    // Canonicalize the path
    let absolute_path = match raw_path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            let cwd = std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "<unknown>".to_string());
            panic!(
                "Failed to resolve path '{}': {e}. \
                Make sure the file or directory exists and is accessible. \
                Current working directory: {cwd}",
                raw_path.display()
            )
        }
    };

    // Convert the path to a string
    let absolute_path_str = absolute_path.to_str().unwrap_or_else(|| {
        panic!(
            "Path '{}' contains invalid UTF-8 characters. \
            This is common on systems with non-UTF-8 file paths. \
            Consider using ASCII-only paths.",
            absolute_path.display()
        )
    });

    // Return the absolute path as a string literal
    let expanded = quote! {
        #absolute_path_str
    };

    TokenStream::from(expanded)
}

/// Check if a path contains suspicious traversal patterns
fn contains_suspicious_patterns(path: &Path) -> bool {
    let mut up_count = 0;
    let mut total_components = 0;

    for component in path.components() {
        total_components += 1;
        if matches!(component, std::path::Component::ParentDir) {
            up_count += 1;
        }
    }

    // Flag as suspicious if more than half the components are '..'
    // or if there are more than 3 consecutive '..' segments
    up_count > 3 || (total_components > 0 && up_count > total_components / 2)
}
