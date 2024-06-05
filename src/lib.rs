#![allow(unreachable_code)]
#![allow(unused_variables)]

mod error;
pub use error::*;

mod core;
mod basex;
mod tests;
mod json;

pub use core::*;
pub use basex::*;
pub use json::*;

#[cfg(all(feature = "serde_json", not(any(feature = "bs58", feature = "base64"))))]
compile_error!("feature `serde_json` required at least one of bs58 and base64");
