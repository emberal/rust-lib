use axum::{
    async_trait,
    extract::{
        multipart::{Field, MultipartError, MultipartRejection},
        FromRequest, Multipart, Request,
    },
    response::IntoResponse,
};
use thiserror::Error;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum ContentType {
    Json,
    Form,
    Multipart,
    Pdf,
    Html,
    Unknown,
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        match content_type {
            "application/json" => ContentType::Json,
            "application/x-www-form-urlencoded" => ContentType::Form,
            "multipart/form-data" => ContentType::Multipart,
            "application/pdf" => ContentType::Pdf,
            "text/html" => ContentType::Html,
            _ => ContentType::Unknown,
        }
    }
}

impl From<String> for ContentType {
    fn from(content_type: String) -> Self {
        ContentType::from(content_type.as_str())
    }
}

impl From<Option<&str>> for ContentType {
    fn from(content_type: Option<&str>) -> Self {
        content_type
            .map(ContentType::from)
            .unwrap_or(ContentType::Unknown)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub content_type: ContentType,
}

impl File {
    pub fn new(
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
        content_type: impl Into<ContentType>,
    ) -> Self {
        Self {
            filename: filename.into(),
            bytes: bytes.into(),
            content_type: content_type.into(),
        }
    }

    async fn from_field(field: Field<'_>) -> Result<Self, MultipartFileRejection> {
        let filename = field
            .file_name()
            .ok_or(MultipartFileRejection::MissingFilename)?
            .to_string();
        let content_type: ContentType = field.content_type().into();
        let bytes = field.bytes().await?;
        Ok(File::new(filename, bytes, content_type))
    }
}

/// Extractor for a single file from a multipart request.
/// Expects exactly one file. A file must have a name, bytes and optionally a content type.
/// This extractor consumes the request and must ble placed last in the handler.
#[derive(Debug, Clone, PartialEq)]
pub struct MultipartFile(pub File);
/// Extractor for multiple files from a multipart request.
/// Expects at least one file. A file must have a name, bytes and optionally a content type.
/// This extractor consumes the request and must ble placed last in the handler.
#[derive(Debug, Clone, PartialEq)]
pub struct MultipartFiles(pub Vec<File>);

#[derive(Debug, Error)]
pub enum MultipartFileRejection {
    #[error(transparent)]
    MultipartRejection(#[from] MultipartRejection),
    #[error("Field error: {0}")]
    FieldError(String),
    #[error("No files found")]
    NoFiles,
    #[error("Expected one file, got several")]
    SeveralFiles,
    #[error("Missing filename")]
    MissingFilename,
    #[error("Error in body of multipart: {0}")]
    BodyError(String),
}

impl From<MultipartError> for MultipartFileRejection {
    fn from(error: MultipartError) -> Self {
        MultipartFileRejection::BodyError(error.body_text())
    }
}

impl IntoResponse for MultipartFileRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            MultipartFileRejection::MultipartRejection(rejection) => rejection.into_response(),
            MultipartFileRejection::FieldError(error) => {
                (axum::http::StatusCode::BAD_REQUEST, error).into_response()
            }
            MultipartFileRejection::NoFiles => {
                (axum::http::StatusCode::BAD_REQUEST, "No files found").into_response()
            }
            MultipartFileRejection::SeveralFiles => (
                axum::http::StatusCode::BAD_REQUEST,
                "Expected one file, got several",
            )
                .into_response(),
            MultipartFileRejection::MissingFilename => {
                (axum::http::StatusCode::BAD_REQUEST, "Missing filename").into_response()
            }
            MultipartFileRejection::BodyError(error) => {
                (axum::http::StatusCode::BAD_REQUEST, error).into_response()
            }
        }
    }
}

#[async_trait]
impl<S> FromRequest<S> for MultipartFile
where
    S: Send + Sync,
{
    type Rejection = MultipartFileRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let multipart = Multipart::from_request(req, state).await?;
        let files = get_files(multipart).await?;
        if files.len() > 1 {
            Err(MultipartFileRejection::SeveralFiles)
        } else {
            let field = files.first().ok_or(MultipartFileRejection::NoFiles)?;
            Ok(MultipartFile(field.clone()))
        }
    }
}

#[async_trait]
impl<S> FromRequest<S> for MultipartFiles
where
    S: Send + Sync,
{
    type Rejection = MultipartFileRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let multipart = Multipart::from_request(req, state).await?;
        let files = get_files(multipart).await?;
        if files.is_empty() {
            Err(MultipartFileRejection::NoFiles)
        } else {
            Ok(MultipartFiles(files))
        }
    }
}

async fn get_files<'a>(mut multipart: Multipart) -> Result<Vec<File>, MultipartFileRejection> {
    let mut files = vec![];
    while let Some(field) = multipart.next_field().await? {
        files.push(File::from_field(field).await?);
    }
    if files.is_empty() {
        Err(MultipartFileRejection::NoFiles)
    } else {
        Ok(files)
    }
}
