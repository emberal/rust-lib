#![allow(dead_code)]

#[cfg(all(feature = "derive", feature = "serde"))]
pub extern crate into_response_derive;
#[cfg(feature = "read-files")]
pub extern crate read_files;

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "nom")]
pub mod nom;
#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "serde")]
pub mod serde;
pub mod traits;
#[cfg(feature = "iter")]
pub mod vector;
