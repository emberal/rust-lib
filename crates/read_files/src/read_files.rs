extern crate proc_macro;

use proc_macro::TokenStream;
use std::{
    collections::HashMap,
    fs::{metadata, read_dir, read_to_string},
    io,
    path::{Path, PathBuf},
};

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Token,
};

pub fn read_files_to_string_impl(args: Args) -> TokenStream {
    let (keys, values) = split_hashmap(args);

    let expanded = quote! {
        {
            let keys = vec![#( #keys, )*];
            let values = vec![#( #values, )*];
            keys.into_iter()
                .zip(values.into_iter())
                .collect::<std::collections::HashMap<&'static str, &'static str>>()
        }
    };
    expanded.into()
}

pub struct Args {
    pub path: String,
    pub pattern: String,
}

struct Syntax {
    path: LitStr,
    /* Comma */
    pattern: Option<LitStr>,
}

impl From<Syntax> for Args {
    fn from(syntax: Syntax) -> Self {
        Self {
            path: syntax.path.value(),
            pattern: syntax
                .pattern
                .map(|pattern| pattern.value())
                .unwrap_or_default(),
        }
    }
}

impl Parse for Args {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        if stream.is_empty() {
            panic!("Expected path argument");
        }

        let path: LitStr = stream.parse()?;
        if path.value().is_empty() {
            panic!("Path must not be empty");
        }

        let pattern = if stream.peek(Token![,]) {
            stream.parse::<Token![,]>()?;
            Some(stream.parse()?)
        } else {
            None
        };

        let syntax = Syntax { path, pattern };
        if !stream.is_empty() {
            panic!("Expected end of input");
        }

        Ok(syntax.into())
    }
}

pub fn split_hashmap(args: Args) -> (Vec<String>, Vec<String>) {
    read_files_to_string(Path::new(&args.path), &args.pattern)
        .unwrap()
        .into_iter()
        .map(|(key, value)| (key.to_string_lossy().to_string(), value))
        .collect()
}

/// Find files within a directory and load them into a HashMap.
/// The key is the file path relative to the root directory.
/// The value is the file contents as a string.
/// # Arguments
/// * `path` - The directory to search for files.
/// * `extension` - The pattern to match files against.
/// # Returns
/// A HashMap containing the file paths and contents.
pub fn read_files_to_string(
    path: &Path,
    pattern: &str,
) -> Result<HashMap<PathBuf, String>, io::Error> {
    use regex::Regex;

    let mut files: HashMap<PathBuf, String> = HashMap::new();
    let dir = read_dir(path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let metadata = metadata(&path)?;

        let regex =
            Regex::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        if metadata.is_file() && regex.is_match(file_name.as_ref()) {
            let file = read_to_string(&path)?;
            files.insert(path, file);
        } else if metadata.is_dir() {
            files.extend(read_files_to_string(&path, pattern)?);
        }
    }
    Ok(files)
}
