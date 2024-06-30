#![allow(dead_code)]

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "nom")]
pub mod nom;
#[cfg(feature = "serde")]
pub mod serde;
pub mod traits;
#[cfg(feature = "iter")]
pub mod vector;

#[cfg(all(feature = "derive", feature = "serde"))]
pub extern crate derive;
