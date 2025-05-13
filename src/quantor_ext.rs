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
//! assert!(xs.forall(|x| *x < 10).is_ok());
//! assert_eq!(xs.select_where(|x| x % 2 == 0), vec![&2]);
//! ```

use std::hash::Hash;

use crate::QuantorError;
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
/// assert!(data.forall(|x| *x < 10).is_ok());
/// ```
pub trait QuantorExt<T> {
    /// Succeeds if all elements satisfy the predicate.
    ///
    /// Returns:
    /// - `Ok(())` if the predicate holds for every element
    /// - `Err(QuantorError::PredicateFailed { index })` on the first violation
    ///
    /// Equivalent to **_∀x ∈ self: pred(x)_**.  
    /// See [`crate::quantifiers::basic::forall`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn forall<F>(&self, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T) -> bool;

    /// Succeeds if any element satisfies the predicate.
    ///
    /// Returns:
    /// - `Ok(())` if at least one element matches
    /// - `Err(QuantorError::NoMatch)` if none do
    ///
    /// Equivalent to **_∃x ∈ self: pred(x)_**.  
    /// See [`crate::quantifiers::basic::exists`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn exists<F>(&self, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T) -> bool;

    /// Succeeds if no element satisfies the predicate.
    ///
    /// Returns:
    /// - `Ok(())` if all elements fail the predicate
    /// - `Err(QuantorError::UnexpectedMatch { index })` on the first match
    ///
    /// Equivalent to **_∀x ∈ self: ¬pred(x)_**.  
    /// See [`crate::quantifiers::basic::none`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn none<F>(&self, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T) -> bool;

    /// Succeeds if exactly one element satisfies the predicate.
    ///
    /// Returns:
    /// - `Ok(())` if one element matches
    /// - `Err(QuantorError::UnexpectedMatch { .. })` if zero or multiple match
    ///
    /// Equivalent to **_∃!x ∈ self: pred(x)_**.  
    /// See [`crate::quantifiers::basic::exactly_one`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn exactly_one<F>(&self, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T) -> bool;

    /// Succeeds if exactly `n` elements satisfy the predicate.
    ///
    /// Returns:
    /// - `Ok(())` if exactly `n` elements match
    /// - `Err(QuantorError::ExactlyNFailed { found, expected })` otherwise
    ///
    /// Equivalent to **_|{x ∈ self | pred(x)}| = n_**.  
    /// See [`crate::quantifiers::basic::exactly_n`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn exactly_n<F>(&self, n: usize, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T) -> bool;

    /// Succeeds if all elements are equal.
    ///
    /// Returns:
    /// - `Ok(())` if every element is equal to the first
    /// - `Err(QuantorError::NotAllEqual { index })` if any differ
    ///
    /// Equivalent to **_∀x, y ∈ self: x = y_**.  
    /// See [`crate::quantifiers::basic::all_equal`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn all_equal(&self) -> Result<(), QuantorError>
    where
        T: Eq;

    /// Succeeds if for every element in `self`, there exists a matching element in `rhs`
    /// such that the predicate holds.
    ///
    /// Returns:
    /// - `Ok(())` if each `self[i]` matches at least one `rhs[j]`
    /// - `Err(QuantorError::ForAllExistsFailed { outer_index })` otherwise
    ///
    /// Equivalent to **_∀x ∈ self ∃y ∈ rhs: pred(x, y)_**.  
    /// See [`crate::quantifiers::nested::forallexists`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn forallexists<U, F>(&self, rhs: &[U], pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T, &U) -> bool;

    /// Succeeds if there exists an element in `self` such that all elements in `rhs`
    /// satisfy the predicate against it.
    ///
    /// Returns:
    /// - `Ok(())` if such an element exists
    /// - `Err(QuantorError::ExistsForAllFailed { outer_index })` otherwise
    ///
    /// Equivalent to **_∃x ∈ self ∀y ∈ rhs: pred(x, y)_**.  
    /// See [`crate::quantifiers::nested::existsforall`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn existsforall<U, F>(&self, rhs: &[U], pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T, &U) -> bool;

    /// Succeeds if the predicate holds for all adjacent pairs.
    ///
    /// Returns:
    /// - `Ok(())` if each pair `(aᵢ, aᵢ₊₁)` satisfies the predicate
    /// - `Err(QuantorError::PairwiseFailed { index })` on the first failure
    ///
    /// Equivalent to **_∀(aᵢ, aᵢ₊₁) ∈ self: pred(aᵢ, aᵢ₊₁)_**.  
    /// See [`crate::quantifiers::structured::pairwise`] for details.
    #[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
    fn pairwise<F>(&self, pred: F) -> Result<(), QuantorError>
    where
        F: Fn(&T, &T) -> bool;

    /// Returns all elements that fail the predicate.
    ///
    /// Equivalent to **_{x ∈ self | ¬pred(x)}_**.  
    /// See [`crate::quantifiers::structured::failing_elements`] for details.
    #[must_use]
    fn failing_elements<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool;

    /// Returns all elements that satisfy the predicate.
    ///
    /// Equivalent to **_{x ∈ self | pred(x)}_**.  
    /// See [`crate::quantifiers::selection::select_where`] for details.
    #[must_use]
    fn select_where<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool;

    /// Returns unique elements that satisfy the predicate.
    ///
    /// Equivalent to **_distinct({x ∈ self | pred(x)})_**.  
    /// See [`crate::quantifiers::selection::select_unique`] for details.
    #[must_use]
    fn select_unique<F>(&self, pred: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
        T: Eq + std::hash::Hash;

    /// Returns all elements that appear more than once.
    ///
    /// Equivalent to **_{x ∈ self | count(x) > 1}_**.  
    /// See [`crate::quantifiers::selection::select_duplicates`] for details.
    #[must_use]
    fn select_duplicates(&self) -> Vec<&T>
    where
        T: Eq + std::hash::Hash;
}

impl<T, S> QuantorExt<T> for S
where
    T: Clone,
    S: AsRef<[T]>, {
    #[inline]
    fn forall<F>(&self, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::forall(self.as_ref(), pred)
    }

    #[inline]
    fn exists<F>(&self, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::exists(self.as_ref(), pred)
    }

    #[inline]
    fn none<F>(&self, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::none(self.as_ref(), pred)
    }

    #[inline]
    fn exactly_one<F>(&self, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::exactly_one(self.as_ref(), pred)
    }

    #[inline]
    fn exactly_n<F>(&self, n: usize, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T) -> bool {
        crate::quantifiers::basic::exactly_n(self.as_ref(), n, pred)
    }

    #[inline]
    fn all_equal(&self) -> Result<(), QuantorError>
    where T: Eq {
        crate::quantifiers::basic::all_equal(self.as_ref())
    }

    #[inline]
    fn forallexists<U, F>(&self, rhs: &[U], pred: F) -> Result<(), QuantorError>
    where F: Fn(&T, &U) -> bool {
        crate::quantifiers::nested::forallexists(self.as_ref(), rhs.iter(), pred)
    }

    #[inline]
    fn existsforall<U, F>(&self, rhs: &[U], pred: F) -> Result<(), QuantorError>
    where F: Fn(&T, &U) -> bool {
        crate::quantifiers::nested::existsforall(self.as_ref(), rhs.iter(), pred)
    }

    #[inline]
    fn pairwise<F>(&self, pred: F) -> Result<(), QuantorError>
    where F: Fn(&T, &T) -> bool {
        crate::quantifiers::structured::pairwise(self.as_ref(), pred)
    }

    #[inline]
    fn failing_elements<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool {
        crate::quantifiers::structured::failing_elements(self.as_ref(), pred)
    }

    #[inline]
    fn select_where<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool {
        crate::quantifiers::selection::select_where(self.as_ref(), pred)
    }

    #[inline]
    fn select_unique<F>(&self, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool,
          T: Eq + Hash {
        crate::quantifiers::selection::select_unique(self.as_ref(), pred)
    }

    #[inline]
    fn select_duplicates(&self) -> Vec<&T>
    where T: Eq + Hash {
        crate::quantifiers::selection::select_duplicates(self.as_ref())
    }
}