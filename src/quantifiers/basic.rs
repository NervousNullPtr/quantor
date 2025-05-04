//! Basic quantifiers such as `forall`, `exists`, `none`, `exactly_one`, and `all_equal`.
//! These functions express simple logical evaluations over a single iterable.
//!
//! Useful in validation, invariant checks, and test assertions.

/// Checks if all elements satisfy the predicate.
/// 
/// Equivalent to **_∀a ∈ iter: pred(a)_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
/// ## Returns
/// * `true` - If all elements satisfy the predicate.
/// * `false` - If there is at least one element that does not satisfy the predicate.
/// ## Example
/// ```
/// use quantor::quantifiers::forall;
/// 
/// let numbers = vec!(0, 2, 4, 6);
/// 
/// assert!(forall(&numbers, |x| x % 2 == 0));
/// ```
#[inline]
pub fn forall<'a, I, T: 'a, F>(iter: I, pred: F) -> bool
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    iter.into_iter().all(pred)
}

/// Checks if at least one element satisfies the predicate.
/// 
/// Equivalent to **_∃a ∈ iter: pred(a)_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
/// ## Returns
/// * `true` if any element satisfies the predicate.
/// * `false` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::exists;
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// 
/// assert!(exists(&numbers, |x| x % 2 == 0));
/// ```
#[inline]
pub fn exists<'a, I, T: 'a, F>(iter: I, pred: F) -> bool
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    iter.into_iter().any(pred)
}

/// Checks if no element satisfies the predicate.
/// 
/// Equivalent to **_∀a ∈ iter: ¬pred(a)_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
/// ## Returns
/// * `true` if no elements satisfy the predicate.
/// * `false` if any do.
///
/// ## Example
/// ```
/// use quantor::quantifiers::none;
/// 
/// let numbers = vec!(1, 3, 5, 7);
/// 
/// assert!(none(&numbers, |x| x % 2 == 0));
/// ```
#[inline]
pub fn none<'a, I, T: 'a, F>(iter: I, pred: F) -> bool
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    !exists(iter, pred)
}

/// Checks if exactly one element satisfies the predicate.
/// 
/// Equivalent to **_∃!a ∈ iter: pred(a)_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// * `pred` - The predicate to test each element against.
/// ## Returns
/// * `true` if exactly one element satisfies the predicate.
/// * `false` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::exactly_one;
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// 
/// assert!(exactly_one(&numbers, |x| x % 2 == 0));
/// ```
#[inline]
pub fn exactly_one<'a, I, T: 'a, F>(iter: I, pred: F) -> bool
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    let mut matched = 0;
    for item in iter {
        if pred(&item) {
            matched += 1;
            if matched > 1 {
                return false;
            }
        }
    }

    matched == 1
}

/// Checks if all elements are equal to each other.
/// 
/// Equivalent to **_∀a,b ∈ iter: a = b_**.
/// ## Arguments
/// * `iter` - The collection to be checked.
/// ## Returns
/// * `true` if all elements are equal to each other.
/// * `false` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::all_equal;
/// 
/// let empty: Vec<usize> = vec!();
/// 
/// let ones    = vec!(1, 1, 1);
/// let natural = vec!(1, 2, 3);
/// 
/// assert!(all_equal(&empty));
/// assert!(all_equal(&ones));
/// assert!(!all_equal(&natural));
/// ```
#[inline]
pub fn all_equal<'a, I, T: 'a>(iter: I) -> bool
where
    I: IntoIterator<Item = &'a T>,
    T: Eq
{
    let mut iter = iter.into_iter();

    iter.next().map_or(true, |first| iter.all(|item| item == first))
}