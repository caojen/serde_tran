mod error;
pub use error::*;

mod core;
mod basex;
mod tests;

pub use core::*;
pub use basex::*;

#[cfg(all(feature = "json", not(any(feature = "bs58", feature = "base64"))))]
compile_error!("feature `json` required at least one of bs58 and base64");
