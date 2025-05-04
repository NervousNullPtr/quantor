//! # quantor
//!
//! `quantor` is a logic-oriented utility crate that provides expressive, composable tools for reasoning over collections.
//!
//! It includes:
//!
//! - Logical quantifiers (`forall`, `exists`, `none`, `exactly_one`, etc.)
//! - Predicate-based selection (`select_where`, `select_unique`, `select_duplicates`)
//! - Structural analysis (`pairwise`, `all_equal`)
//! - Counting and duplicate detection (`counter`)
//! - Assertion macros (`assert_forall!`, `assert_unique!`, etc.)
//!
//! This crate focuses on enabling clear, declarative logic over iterables.
//!
//! ## Example
//! ```
//! use quantor::{forall, select_where};
//!
//! let values = vec![2, 4, 6];
//! 
//! assert!(forall(&values, |x| x % 2 == 0).is_ok());
//! 
//! let evens = select_where(&values, |x| x % 2 == 0);
//! assert_eq!(evens.as_slice(), [&2, &4, &6]);
//! ```
//!
//! All tools are generic over any `IntoIterator`, making them flexible across slices, vectors, sets, and more.

#[macro_use]
mod macros;

pub mod error;
pub use error::QuantorError;

pub mod quantifiers;
pub mod prelude;

#[cfg(feature = "method-api")]
pub mod quantor_ext;

pub use quantifiers::*;
#[cfg(feature = "method-api")]
pub use quantor_ext::*;