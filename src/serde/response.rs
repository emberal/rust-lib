#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Serialize)]
#[cfg(feature = "serde")]
pub struct BaseResponse<T: Serialize> {
    pub version: String,
    #[serde(flatten)]
    pub body: T, // T must be a struct (or enum?)
}

#[cfg(feature = "serde")]
impl<T: Serialize> BaseResponse<T> {
    pub fn new(version: impl Into<String>, body: T) -> Self {
        Self {
            version: version.into(),
            body,
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Response {
        message: String,
    }

    #[test]
    fn test_base_response_new() {
        let response = BaseResponse::new(
            "",
            Response {
                message: "Hi".to_string(),
            },
        );
        assert_eq!(response.body.message, "Hi".to_string());
    }
}
