//! # quantor: QuantorExt
//!
//! This module defines [`QuantorExt`], a trait that provides method-style access to all logical
//! quantifier operations available in the `quantor` crate.
//!
//! Instead of using the standalone functions like `forall(&numbers, pred)`, this trait allows you
//! to write more fluent expressions such as `numbers.forall(pred)` or `numbers.select_where(...)`.
//!
//! The trait is implemented for any type that can be referenced as a slice (`AsRef<[T]>`),
//! making it compatible with `Vec<T>`, slices, arrays, and similar types.
//!
//! ## Included Quantifiers
//! - Core: `forall`, `exists`, `none`, `exactly_one`, `all_equal`
//! - Nested: `forallexists`, `existsforall`
//! - Structured: `pairwise`, `failing_elements`
//! - Selection: `select_where`, `select_unique`, `select_duplicates`
//!
//! Enable the `method-api` feature to activate this module and import it via `quantor::prelude::*`.
//!
//! ## Example
//! ```
//! use quantor::prelude::*;
//!
//! let xs = vec![1, 2, 3];
//! assert!(xs.forall(|x| *x < 10));
//! assert_eq!(xs.select_where(|x| x % 2 == 0), vec![&2]);
//! ```

use std::hash::Hash;
/// Extension trait providing method-style quantifier logic over collections.
///
/// This trait enables calling quantifiers like `forall`, `exists`, `none`, and selection
/// methods like `select_where`, `select_unique`, and `select_duplicates` directly on
/// any type that implements `AsRef<[T]>`, such as `Vec<T>`, slices, and arrays.
///
/// Each method corresponds to a function in the `crate::quantifiers` module, preserving
/// the same semantics and guarantees while offering a more fluent and expressive syntax.
///
/// To use this trait, import it via the `quantor::prelude::*` module or enable the `method-api` feature.
///
/// ## Example
/// ```
/// use quantor::prelude::*;
///
/// let data = vec![1, 2, 3];
/// assert!(data.forall(|x| *x < 10));
/// ```
pub trait QuantorExt<T> {
    /// Returns `true` if all elements satisfy the predicate.
    ///
    /// Equivalent to **_∀x ∈ self: pred(x)_**.
    ///
    /// See [`crate::quantifiers::basic::forall`] for details.
    fn forall<F>(&self, pred: F) -> bool
    where
        F: Fn(&T) -> bool;

    /// Returns `true` if any element satisfies the predicate.
    ///
    /// Equivalent to **_∃x ∈ self: pred(x)_**.
    ///
    /// See [`crate::quantifiers::basic::exists`] for details.
    fn exists<F>(&self, pred: F) -> bool
    where
        F: Fn(&T) -> bool;

    /// Returns `true` if no element satisfies the predicate.
    ///
    /// Equivalent to **_∀x ∈ self: ¬pred(x)_**.
    ///
    /// See [`crate::quantifiers::basic::none`] for details.
    fn none<F>(&self, pred: F) -> bool
    where
        F: Fn(&T) -> bool;

    /// Returns `true` if exactly one element satisfies the predicate.
    ///
    /// Equivalent to **_∃!x ∈ self: pred(x)_**.
    ///
    /// See [`crate::quantifiers::basic::exactly_one`] for details.
    fn exactly_one<F>(&self, pred: F) -> bool
    where
        F: Fn(&T) -> bool;

    /// Returns `true` if all elements are equal to each other.
    ///
    /// Equivalent to **_∀x, y ∈ self: x = y_**.
    ///
    /// See [`crate::quantifiers::basic::all_equal`] for details.
    fn all_equal(&self) -> bool
    where
        T: Eq;

    /// Returns `true` if for every element in `self`, there exists a matching element in `rhs`
    /// such that the predicate holds.
    ///
    /// Equivalent to **_∀x ∈ self ∃y ∈ rhs: pred(x, y)_**.
    ///
    /// See [`crate::quantifiers::nested::forallexists`] for details.
    fn forallexists<U, F>(&self, rhs: &[U], pred: F) -> bool
    where
        F: Fn(&T, &U) -> bool;

    /// Returns `true` if there exists an element in `self` such that all elements in `rhs`
    /// satisfy the predicate.
    ///
    /// Equivalent to **_∃x ∈ self ∀y ∈ rhs: pred(x, y)_**.
    ///
    /// See [`crate::quantifiers::nested::existsforall`] for details.
    fn existsforall<U, F>(&self, rhs: &[U], pred: F) -> bool
    where
        F: Fn(&T, &U) -> bool;

    /// Returns `true` if the predicate holds for all adjacent pairs.
    ///
    /// Equivalent to **_∀(aᵢ, aᵢ₊₁) ∈ self: pred(aᵢ, aᵢ₊₁)_**.
    ///
    /// See [`crate::quantifiers::structured::pairwise`] for details.
    fn pairwise<F>(&self, pred: F) -> bool
    where
        F: Fn(&T, &T) -> bool;

    /// Returns all elements that do **not** satisfy the predicate.
    ///
    /// Equivalent to **_{x ∈ self | ¬pred(x)}_**.
    ///
    /// See [`crate::quantifiers::structured::failing_elements`] for details.
    fn failing_elements<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool;

    /// Returns all elements that satisfy the predicate.
    ///
    /// Equivalent to **_{x ∈ self | pred(x)}_**.
    ///
    /// See [`crate::quantifiers::selection::select_where`] for details.
    fn select_where<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool;

    /// Returns unique elements that satisfy the predicate.
    ///
    /// Equivalent to **_distinct({x ∈ self | pred(x)})_**.
    ///
    /// See [`crate::quantifiers::selection::select_unique`] for details.
    fn select_unique<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
        T: Eq + std::hash::Hash;

    /// Returns all duplicated elements in the collection.
    ///
    /// Equivalent to **_{x ∈ self | count(x) > 1}_**.
    ///
    /// See [`crate::quantifiers::selection::select_duplicates`] for details.
    fn select_duplicates(&self) -> Vec<&T>
    where
        T: Eq + std::hash::Hash;
}

impl<T, S> QuantorExt<T> for S
where
    T: Clone,
    S: AsRef<[T]>, {
    fn forall<F>(&self, pred: F) -> bool
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::forall(self.as_ref(), pred)
    }

    fn exists<F>(&self, pred: F) -> bool
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::exists(self.as_ref(), pred)
    }

    fn none<F>(&self, pred: F) -> bool
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::none(self.as_ref(), pred)
    }

    fn exactly_one<F>(&self, pred: F) -> bool
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::exactly_one(self.as_ref(), pred)
    }

    fn all_equal(&self) -> bool
    where T: Eq {
        crate::quantifiers::basic::all_equal(self.as_ref())
    }

    fn forallexists<U, F>(&self, rhs: &[U], pred: F) -> bool
    where F: Fn(&T, &U) -> bool {
        crate::quantifiers::nested::forallexists(self.as_ref(), rhs.iter(), pred)
    }

    fn existsforall<U, F>(&self, rhs: &[U], pred: F) -> bool
    where F: Fn(&T, &U) -> bool {
        crate::quantifiers::nested::existsforall(self.as_ref(), rhs.iter(), pred)
    }

    fn pairwise<F>(&self, pred: F) -> bool
    where F: Fn(&T, &T) -> bool {
        crate::quantifiers::structured::pairwise(self.as_ref(), pred)
    }

    fn failing_elements<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool {
        crate::quantifiers::structured::failing_elements(self.as_ref(), pred)
    }

    fn select_where<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool {
        crate::quantifiers::selection::select_where(self.as_ref(), pred)
    }

    fn select_unique<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool,
          T: Eq + Hash {
        crate::quantifiers::selection::select_unique(self.as_ref(), pred)
    }

    fn select_duplicates(&self) -> Vec<&T>
    where T: Eq + Hash {
        crate::quantifiers::selection::select_duplicates(self.as_ref())
    }
}