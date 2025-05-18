//! Common traits and functions for quick access.
//!
//! Includes all quantifiers and extension traits.
//! Import this to bring the full quantor interface into scope.

pub use crate::quantifiers::basic::*;
pub use crate::quantifiers::nested::*;
pub use crate::quantifiers::structured::*;
pub use crate::quantifiers::selection::*;

#[cfg(feature = "method-api")]
pub use crate::quantor_ext::QuantorExt;