use axum::response::{IntoResponse, Response};
use derive_more::{Constructor, From};
use into_response_derive::IntoResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, From, Constructor)]
pub struct Array<T: Serialize> {
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Copy, Serialize, IntoResponse, From, Constructor)]
pub struct Count {
    pub count: usize,
}

impl<T: Serialize> IntoResponse for Array<T> {
    fn into_response(self) -> Response {
        crate::from!(self).into_response()
    }
}
