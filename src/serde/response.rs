use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
    pub version: String,
    #[serde(flatten)]
    pub body: T, // T must be a struct (or enum?) TODO from! macro that validates T on compile time
}

impl<T: Serialize> BaseResponse<T> {
    pub fn new(version: impl Into<String>, body: T) -> Self {
        Self {
            version: version.into(),
            body,
        }
    }
}

#[macro_export]
macro_rules! from {
    ($body:expr) => {
        $crate::serde::response::BaseResponse::new(env!("CARGO_PKG_VERSION"), $body)
    };
}

#[cfg(test)]
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

    #[test]
    fn test_from_macro() {
        let response = from!(Response {
            message: "Hi".to_string(),
        });
        from!(1); // Should not be allowed
        assert_eq!(response.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(response.body.message, "Hi".to_string());
    }
}
