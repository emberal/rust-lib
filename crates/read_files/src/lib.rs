extern crate proc_macro;
use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::read_files::read_files_to_string_impl;

mod read_files;

/// Read files from a directory into a HashMap.
/// The key is the file path relative to the root directory.
/// The value is the file contents as a string.
/// # Arguments
/// * `path` - The directory to search for files, relative to the root directory.
/// * `pattern` - The regex pattern to match files against. If missing, all files are matched.
/// # Returns
/// A HashMap containing the file paths and contents.
/// # Example
/// ```
/// use read_files::read_files_to_string;
///
/// let files = read_files_to_string!("./src", ".rs$");
/// assert!(!files.is_empty());
/// ```
/// # Panics
/// If the path is empty. \
/// If the pattern is invalid. \
/// If the path does not exist. \
/// If there are unexpected tokens. \
#[proc_macro]
pub fn read_files_to_string(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as read_files::Args);
    read_files_to_string_impl(args)
}
