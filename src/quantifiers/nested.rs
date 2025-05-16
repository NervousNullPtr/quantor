//! Nested quantifiers involving two-level logic.
//!
//! These are helpful for modeling containment, dominance, or existential constraints in rule engines.

use crate::QuantorError;

/// Checks whether for every element in `a`, there exists at least one element in `b` for which the predicate holds.
/// 
/// Equivalent to **_∀x ∈ a ∃y ∈ b: pred(x, y)_**.
/// 
/// **Note**: _If `b` is empty and `a` is not, this will always return an error._
/// ## Arguments
/// - `a` - The source collection (outer quantifier).
/// - `b` - The comparison collection.
/// - `pred` - The binary predicate to check against.
/// ## Returns
/// - `Ok(())` if every element in the left-hand collection (`a`) satisfies the condition against at least one element in the right-hand collection (`b`).
/// - `Err(QuantorError::ForAllExistsFailed { outer_index })` if any `a` fails to match.
/// ## Example
/// ```
/// use quantor::{quantifiers::forallexists, error::QuantorResultExt};
///
/// let a = vec![1, 2];
/// let b = vec![2, 3, 4];
///
/// assert!(forallexists(&a, &b, |x, y| x < y).is_ok());
///
/// let failing = vec![10, 20];
/// let err = forallexists(&failing, &b, |x, y| x < y);
///
/// if let Some(index) = err.failing_index() {
///     assert_eq!(0, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn forallexists<'a, A: 'a, B: 'a>(
    a: impl IntoIterator<Item = &'a A>,
    b: impl IntoIterator<Item = &'a B>,
    pred: impl Fn(&A, &B) -> bool,
) -> Result<(), QuantorError> {

    let b_vec: Vec<&'a B> = b.into_iter().collect();
    let mut matched;

    for (outer_index, item_a) in a.into_iter().enumerate() {
        matched = false;

        for item_b in &b_vec {
            if pred(item_a, item_b) {
                matched = true;
                break;
            }
        }

        if !matched {
            return Err(QuantorError::ForAllExistsFailed { outer_index });
        }
    }

    Ok(())
}

/// Checks whether there exists an element in `a` for which all elements in `b` satisfy the predicate.
/// 
/// Equivalent to **_∃x ∈ a ∀y ∈ b: pred(x, y)_**.
/// ## Arguments
/// - `a` - The source collection (outer quantifier).
/// - `b` - The comparison collection.
/// - `pred` - The binary predicate to check against.
/// ## Returns
/// - `Ok(())` if there exists at least one element in the left-hand collection (`a`) such that the condition holds against all elements in the right-hand collection (`b`).
/// - `Err(QuantorError::ExistsForAllFailed { outer_index })` if no such element exists, where `outer_index` is the index of the first failing `a` element.
/// ## Example
/// ```
/// use quantor::{quantifiers::existsforall, error::QuantorResultExt};
///
/// let a = vec![5, 10];
/// let b = vec![1, 2];
///
/// assert!(existsforall(&a, &b, |x, y| x > y).is_ok());
///
/// let failing = vec![0, 1];
/// let err = existsforall(&failing, &b, |x, y| x > y);
///
/// if let Some(index) = err.failing_index() {
///     assert_eq!(0, index);
/// }
/// ```
#[inline]
#[must_use = "Quantifier results must be checked. Use `.is_ok()` or `?` to handle them."]
pub fn existsforall<'a, A: 'a, B: 'a>(
    a: impl IntoIterator<Item = &'a A>,
    b: impl IntoIterator<Item = &'a B>,
    pred: impl Fn(&A, &B) -> bool,
) -> Result<(), QuantorError> {
    let b_vec: Vec<&'a B> = b.into_iter().collect();
    let mut first_index = None;

    for (index, item_a) in a.into_iter().enumerate() {
        let mut all_match = true;

        for item_b in &b_vec {
            if !pred(item_a, item_b) {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Ok(());
        }

        if first_index.is_none() {
            first_index = Some(index)
        }
    }

    Err(QuantorError::ExistsForAllFailed { outer_index: first_index.unwrap_or(0) })
}