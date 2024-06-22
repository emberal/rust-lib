#[cfg(feature = "tokio")]
use {std::io::Error, tokio::fs::File, tokio_util::io::ReaderStream};

#[cfg(feature = "tokio")]
pub async fn load_file<Path>(file_path: Path) -> Result<ReaderStream<File>, Error>
where
    Path: AsRef<std::path::Path>,
{
    File::open(file_path).await.map(ReaderStream::new)
}

#[cfg(all(test, feature = "tokio"))]
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
