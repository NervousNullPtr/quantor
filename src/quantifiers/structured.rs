//! Predicate-based selection utilities.
//!
//! These functions are useful in areas such as reporting or constrained filtering.

/// Checks whether a binary predicate holds for all adjacent pairs.
/// 
/// Equivalent to: **∀(aᵢ, aᵢ₊₁) ∈ self: pred(aᵢ, aᵢ₊₁)**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
/// ## Example
/// ```
/// use quantor::quantifiers::pairwise;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert!(pairwise(&numbers, |a, b| a < b));
/// ```
#[inline]
pub fn pairwise<'a, I, T: 'a, F>(iter: I, pred: F) -> bool
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T, &T) -> bool,
{
    let mut iter = iter.into_iter();

    let mut prev = match iter.next() {
        Some(p) => p,
        _       => { return true; }
    };

    for curr in iter {
        if !pred(&prev, &curr) {
            return false;
        }
        prev = curr;
    }

    true
}
/// Gets all elements that fail the predicate.
/// 
/// Equivalent to **_{x ∈ self | ¬pred(x)}_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
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
pub fn failing_elements<'a, I, T: 'a, F>(iter: I, pred: F) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    iter.into_iter().filter(|x| !pred(x)).collect()
}