#![allow(dead_code)]
extern crate self as lib;

#[cfg(all(feature = "derive", feature = "diesel"))]
pub extern crate diesel_crud_derive;
#[cfg(feature = "diesel")]
pub extern crate diesel_crud_trait;
#[cfg(all(feature = "derive", feature = "axum", feature = "serde"))]
pub extern crate into_response_derive;
#[cfg(feature = "read-files")]
pub extern crate read_files;

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "nom")]
pub mod nom;
#[cfg(feature = "serde")]
pub mod serde;
#[cfg(feature = "time")]
pub mod time;
pub mod traits;
#[cfg(feature = "iter")]
pub mod vector;
