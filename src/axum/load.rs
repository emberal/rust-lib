#[cfg(feature = "io")]
use {crate::io::file, axum::body::Body, axum::response::Html, std::io};

/// Load an HTML file from the given file path, relative to the current directory.
/// # Arguments
/// * `file_path` - The path to the HTML file.
/// # Returns
/// The HTML file as a `Html` object containing the content-type 'text/html' or an error message if the file is not found or cannot be read.
/// # Examples
/// ```
/// let html = async { lib::axum::load::load_html("openapi.html").await.unwrap() };
/// ```
#[cfg(feature = "io")]
pub async fn load_html<Path>(file_path: Path) -> Result<Html<Body>, io::Error>
where
    Path: AsRef<std::path::Path>,
{
    load_file(file_path).await.map(Html)
}

#[cfg(feature = "io")]
pub async fn load_file<Path>(file_path: Path) -> Result<Body, io::Error>
where
    Path: AsRef<std::path::Path>,
{
    file::load_file(file_path).await.map(Body::from_stream)
}

/// Load an HTML file from the given file path, relative to the resource directory.
/// The file is loading on compile time as a string literal.
/// # Arguments
/// * `filename` - The path to the HTML file.
/// # Returns
/// The HTML file as a `Html` object containing the content-type 'text/html'.
/// # Examples
/// ```
/// let _html = lib::load_html!("load.rs");
/// ```
// TODO check platform and use correct path separator
#[macro_export]
macro_rules! load_html {
    ($filepath:expr) => {
        axum::response::Html(
            axum::body::Body::new(
                include_str!($filepath).to_string()
            )
        )
    };
    ($filepath:expr, $($key:expr => $value:expr),*) => {
        axum::response::Html(
            axum::body::Body::new(
                include_str!($filepath)$(
                    .replace($key, $value)
                )*
            )
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_html() {
        let _html = load_html!("load.rs");
    }

    #[test]
    fn test_load_html_with_replacements() {
        let _html =
            load_html!("load.rs", "{{replace_me}}" => "hello", "{{replace_me_too}}" => "world");
    }

    #[cfg(feature = "io")]
    mod tokio {
        use super::super::*;

        #[tokio::test]
        async fn test_load_html() {
            assert!(load_html("Cargo.toml").await.is_ok());
        }

        #[tokio::test]
        async fn test_load_file() {
            assert!(load_file("Cargo.toml").await.is_ok());
        }

        #[tokio::test]
        async fn test_load_file_not_found() {
            assert!(load_file("not_found.rs").await.is_err());
        }
    }
}
