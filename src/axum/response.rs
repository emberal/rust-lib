use {
    crate::serde::response::BaseResponse,
    axum::{
        response::{IntoResponse, Response},
        Json,
    },
    serde::Serialize,
};

impl<T: Serialize> IntoResponse for BaseResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::header::CONTENT_TYPE;
    use axum::http::{HeaderValue, StatusCode};
    use axum::response::IntoResponse;
    use mime::APPLICATION_JSON;
    use serde::Serialize;

    use crate::serde::response::BaseResponse;

    #[derive(Serialize)]
    struct Response {
        message: String,
    }

    #[test]
    fn test_into_response() {
        let response = BaseResponse::new(
            "",
            Response {
                message: "Hi".to_string(),
            },
        );
        let json_response = response.into_response();
        assert_eq!(json_response.status(), StatusCode::OK);
        assert_eq!(
            json_response.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static(APPLICATION_JSON.as_ref()))
        );
    }

    #[test]
    fn test_into_response_with_primitive() {
        let response = BaseResponse::new("", 42);
        assert_eq!(
            response.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
