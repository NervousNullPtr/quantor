//! # quantor: Macros
//! This module defines declarative macros for logical quantification in a mathematical syntax.
//! These macros provide syntactic sugar over the core quantifier functions, allowing for more expressive
//! and readable logic in test code, runtime checks, and validation layers.
//!
//! ## Core Macro
//! - [`quantify!`] — A unified macro that supports all core quantifier forms, using concise math-inspired syntax:
//!
//! ```
//! use quantor::quantify;
//!
//! let numbers = vec![1, 2, 3];
//! assert!(quantify!(forall x in (&numbers) => *x > 0).is_ok());
//! assert!(quantify!(exists x in (&numbers) => *x == 2).is_ok());
//! ```
//!
//! For validation-specific macros like `assert_forall!` or `assert_duplicates!`, see the [`assertions`](crate::assertions) module.

mod select;
mod quantify;
mod assertions;

#[cfg(feature = "debug-tools")]
mod debug;