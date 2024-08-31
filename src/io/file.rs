use {std::io::Error, tokio::fs::File, tokio_util::io::ReaderStream};

/// Loads a file from the file system and returns a stream of bytes.
/// # Arguments
/// * `file_path` - The path to the file to load.
/// # Returns
/// A stream of bytes from the file if the file is found. Otherwise, an error is returned.
pub async fn load_file<Path>(file_path: Path) -> Result<ReaderStream<File>, Error>
where
    Path: AsRef<std::path::Path>,
{
    File::open(file_path).await.map(ReaderStream::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_file() {
        let file = load_file("Cargo.toml").await;
        assert!(file.is_ok());
    }

    #[tokio::test]
    async fn test_load_file_error() {
        let file = load_file("Cargo.tom").await;
        assert!(file.is_err());
    }
}
