//! Basic quantifiers such as `forall`, `exists`, `none`, `exactly_one`, and `all_equal`.
//! These functions express simple logical evaluations over a single iterable.
//!
//! Useful in validation, invariant checks, and test assertions.

use crate::{error::{QuantorKind}, QuantorError};

/// Checks if all elements satisfy the predicate.
/// 
/// Equivalent to **_∀a ∈ iter: pred(a)_**.
/// When `a` is `∅`, this returns `ok(())` because there is no counterexample.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if all elements satisfy the predicate.
/// - `Err(QuantorError::PredicateFailed { index })` if an element fails the predicate, with the index of the first failure.
/// ## Example
/// ```
/// use quantor::{quantifiers::forall, error::QuantorResultExt};
/// 
/// let numbers = vec!(0, 2, 4, 6);
/// assert!(forall(&numbers, |x| x % 2 == 0).is_ok());
/// 
/// let bad = vec!(0, 1, 2, 4, 6);
/// let err = forall(&bad, |x| x % 2 == 0);
/// 
/// if let Some(index) = err.failing_index() {
///     assert_eq!(1, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn forall<'a, I, T: 'a, F>(iter: I, pred: F) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    for (i, item) in iter.into_iter().enumerate() {
        if !pred(item) {
            return Err(QuantorError::PredicateFailed { kind: QuantorKind::Forall, index: i })
        }
    }

    Ok(())
}

/// Checks if at least one element satisfies the predicate.
/// 
/// Equivalent to **_∃a ∈ iter: pred(a)_**.
/// When `a` is `∅`, this returns `QuantorError` because there is no element that can satisfy `pred`.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if any element satisfies the predicate.
/// - `Err(QuantorError::NoMatch)` if no element satisfies the predicate.
/// ## Example
/// ```
/// use quantor::quantifiers::exists;
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// assert!(exists(&numbers, |x| x % 2 == 0).is_ok());
/// 
/// let bad = vec!(1, 3, 5, 7);
/// assert!(exists(&bad, |x| x % 2 == 0).is_err());
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn exists<'a, I, T: 'a, F>(iter: I, pred: F) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    for item in iter {
        if pred(item) {
            return Ok(());
        }
    }

    Err(QuantorError::NoMatch {kind: QuantorKind::Exists})
}

/// Checks if no element satisfies the predicate.
/// 
/// Equivalent to **_∀a ∈ iter: ¬pred(a)_**.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if no elements satisfy the predicate.
/// - `Err(QuantorError::UnexpectedMatch { index })` if at least one element satisfies the predicate, with the `index`.
///
/// ## Example
/// ```
/// use quantor::{quantifiers::none, error::QuantorResultExt};
/// 
/// let numbers = vec!(1, 3, 5, 7);
/// assert!(none(&numbers, |x| x % 2 == 0).is_ok());
/// 
/// let bad = vec!(1, 3, 5, 7, 0);
/// let err = none(&bad, |x| x % 2 == 0);
/// 
/// if let Some(index) = err.failing_index() {
///     assert_eq!(4, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn none<'a, I, T: 'a, F>(iter: I, pred: F) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    for (index, item) in iter.into_iter().enumerate() {
        if pred(item) {
            return Err(QuantorError::UnexpectedMatch { kind: QuantorKind::None, index });
        }
    }

    Ok(())
}

/// Checks if exactly one element satisfies the predicate.
/// 
/// Equivalent to **_∃!a ∈ iter: pred(a)_**.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if exactly one element satisfies the predicate.
/// - `Err(QuantorError::UnexpectedMatch { index })` when there is more than one element which satisfies the predicate, with the `index` of the second passing element.
/// ## Example
/// ```
/// use quantor::{quantifiers::exactly_one, error::QuantorResultExt};
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// assert!(exactly_one(&numbers, |x| x % 2 == 0).is_ok());
/// 
/// let bad = vec!(0, 1, 3, 5, 6);
/// let err = exactly_one(&bad, |x| x % 2 == 0);
/// 
/// if let Some(index) = err.failing_index() {
///     assert_eq!(4, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn exactly_one<'a, I, T: 'a, F>(iter: I, pred: F) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    let mut iter = iter.into_iter().enumerate().peekable();

    // Check for empty input
    if iter.peek().is_none() {
        return Err(QuantorError::EmptyInput {
            kind: QuantorKind::ExactlyOne,
        });
    }

    let mut matched = 0;

    for (index, item) in iter {
        if pred(item) {
            matched += 1;
            if matched > 1 {
                return Err(QuantorError::UnexpectedMatch { kind: QuantorKind::ExactlyOne, index });
            }
        }
    }

    if matched == 1 {
        Ok(())
    } else {
        Err(QuantorError::PredicateFailed {
            kind: QuantorKind::ExactlyOne,
            index: 0,
        })
    }
}

/// Checks if all elements are equal to each other.
/// 
/// Equivalent to **_∀a,b ∈ iter: a = b_**.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// ## Returns
/// - `Ok(())` if all elements are equal to each other.
/// - `Err(QuantorError::NotAllEqual { index })` if an element at `index` is not equal to the first element.
/// ## Example
/// ```
/// use quantor::{quantifiers::all_equal, error::QuantorResultExt};
/// 
/// let empty: Vec<usize> = vec!();
/// 
/// let ones    = vec!(1, 1, 1);
/// let natural = vec!(1, 2, 3);
/// 
/// assert!(all_equal(&empty).is_ok());
/// assert!(all_equal(&ones).is_ok());
/// 
/// let err = all_equal(&natural);
/// 
/// if let Some(index) = err.failing_index() {
///     assert_eq!(1, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn all_equal<'a, I, T>(iter: I) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Eq
{
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        for (i, item) in iter.enumerate() {
            if item != first {
                return Err(QuantorError::NotAllEqual { kind: QuantorKind::AllEqual, index: i + 1 });
            }
        }
    }

    Ok(())
}

/// Checks if exactly `n` elements in the iterator satisfy the predicate.
///
/// Equivalent to **_|{x ∈ iter | pred(x)}| = n_**
///
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `n` - The number of assumed elements to satisfy `pred`.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if exactly `n` elements match.
/// - `Err(QuantorError::ExactlyNFailed { found, expected })` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::exactly_n;
/// use quantor::error::QuantorResultExt;
///
/// let values = vec![1, 2, 4, 6];
/// let result = exactly_n(&values, 3, |x| x % 2 == 0);
///
/// assert!(result.is_ok());
///
/// let err = exactly_n(&values, 2, |x| x % 2 == 0);
///
/// if let Some(expected) = err.match_count() {
///     assert_eq!(expected, 3);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn exactly_n<'a, I, T: 'a, F>(
    iter: I,
    n: usize,
    pred: F,
) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    let found = iter.into_iter().filter(|x| pred(x)).count();

    if found == n {
        Ok(())
    } else {
        Err(QuantorError::ExactlyNFailed { kind: QuantorKind::ExactlyN, found, expected: n })
    }
}