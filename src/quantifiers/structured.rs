//! Predicate-based selection utilities.
//!
//! These functions are useful in areas such as reporting or constrained filtering.

use crate::QuantorError;

/// Checks whether a binary predicate holds for all adjacent pairs.
/// 
/// Equivalent to: **∀(aᵢ, aᵢ₊₁) ∈ self: pred(aᵢ, aᵢ₊₁)**.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// - `Ok(())` if the predicate holds for all adjacent pairs.
/// - `Err(QuantorError::PairwiseFailed { index })` if any pair violates the predicate.
/// ## Example
/// ```
/// use quantor::quantifiers::pairwise;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert!(pairwise(&numbers, |a, b| a < b).is_ok());
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn pairwise<'a, I, T: 'a, F>(iter: I, pred: F) -> Result<(), QuantorError>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T, &T) -> bool,
{
    let mut iter = iter.into_iter();
    let mut prev = match iter.next() {
        Some(p) => p,
        None => return Ok(())
    };

    for (i, curr) in iter.enumerate() {
        if !pred(prev, curr) {
            // Index `i` here refers to the second item in the failing pair.
            return Err(QuantorError::PairwiseFailed { index: i });
        }
        prev = curr;
    }

    Ok(())
}
/// Gets all elements that fail the predicate.
/// 
/// Equivalent to **_{x ∈ self | ¬pred(x)}_**.
/// ## Arguments
/// - `iter` - The collection to be checked.
/// - `pred` - The predicate to test each element against.
/// ## Returns
/// A `Vec<&T>` containing all elements which failed the predicate.
/// ## Example
/// ```
/// use quantor::quantifiers::failing_elements;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert_eq!(failing_elements(&numbers, |x| x % 2 == 0), vec!(&1, &3));
/// ```
#[inline]
#[must_use = "Use this to inspect failing elements from a quantifier."]
pub fn failing_elements<'a, I, T: 'a, F>(iter: I, pred: F) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    iter.into_iter().filter(|x| !pred(x)).collect()
}