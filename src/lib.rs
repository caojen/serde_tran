mod error;
pub use error::*;

mod core;
mod basex;
mod tests;

pub use core::*;
pub use basex::*;

#[cfg(all(feature = "base64", feature = "bs58"))]
compile_error!("At most one of feature `base64` and feature `bs58` can be enabled");
