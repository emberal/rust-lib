use axum::{
    async_trait,
    extract::{
        multipart::{Field, MultipartError, MultipartRejection},
        FromRequest, Multipart, Request,
    },
    response::IntoResponse,
};
use mime::Mime;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub content_type: Mime,
}

impl File {
    pub fn new(
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
        content_type: impl Into<Mime>,
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
        let content_type = Mime::from_str(field.content_type().ok_or_else(|| {
            MultipartFileRejection::FieldError("Missing or illegal content type".to_string())
        })?)?;
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
    #[error(transparent)]
    FromStrError(#[from] mime::FromStrError),
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
            MultipartFileRejection::FromStrError(error) => {
                (axum::http::StatusCode::BAD_REQUEST, error.to_string()).into_response()
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
