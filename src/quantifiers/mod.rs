//! # quantor: Quantifiers
//! This module defines core logical operations over iterables, allowing you to express and evaluate
//! quantifier-style logic in a clear, declarative, and composable way. All quantifiers are implemented
//! generically using `IntoIterator`, making them compatible with slices, vectors, and other iterable types.
//!
//! The module is split into logical categories:
//!
//! - [`core`] – Fundamental quantifiers such as `forall`, `exists`, and `none`.
//! - [`nested`] – Nested quantifier constructs such as `forallexists` and `existsforall`.
//! - [`selection`] – Selection utilities based on predicates, like `select_where` or `select_unique`.
//! - [`structured`] – Structure-oriented logic, such as pairwise conditions or equality across elements.
//!
//! These tools work well with the predicates module and assertion macros to support logical validation and filtering.

pub mod basic;
pub mod nested;
pub mod selection;
pub mod structured;

pub use basic::*;
pub use nested::*;
pub use selection::*;
pub use structured::*;
