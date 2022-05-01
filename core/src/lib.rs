#![cfg_attr(feature = "nightly", feature(specialization))]
//! Core structs and traits for paperclip.

#[cfg(feature = "actix2")]
extern crate actix_web2 as actix_web;
#[cfg(feature = "actix3")]
extern crate actix_web3 as actix_web;
#[cfg(feature = "actix4")]
extern crate actix_web4 as actix_web;
#[cfg_attr(feature = "v2", macro_use)]
extern crate serde;

mod error;
pub mod common;
pub mod extensions;
pub mod im;
pub mod util;
pub mod version;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;

pub use self::error::ValidationError;
pub use self::common::{HttpMethod, Either, SpecFormat};
pub use self::version::Version;
pub use self::extensions::MediaRange;

#[cfg(all(feature = "actix2", feature = "actix3"))]
compile_error!("feature \"actix2\" and feature \"actix3\" cannot be enabled at the same time");

#[cfg(all(feature = "actix3", feature = "actix4"))]
compile_error!("feature \"actix3\" and feature \"actix4\" cannot be enabled at the same time");

#[cfg(all(feature = "actix2", feature = "actix4"))]
compile_error!("feature \"actix2\" and feature \"actix4\" cannot be enabled at the same time");
