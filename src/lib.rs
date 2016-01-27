//! Rust implementations of algorithms from
//! Introduction to Algorithms by Thomas Cormen, et al.

#![crate_name = "cormen_rust"]
#![crate_type = "lib"]

pub use self::utils::*;
pub use self::sorts::*;

mod utils;
mod sorts;