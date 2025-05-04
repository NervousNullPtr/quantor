//! # quantor: Assertions
//!
//! This module provides macros for validating logical properties of sequences at runtime.
//! All macros panic on failure and are suitable for use in tests or precondition enforcement.
//!
//! ## Available Macros
//!
//! - [`assert_forall!`] - Ensures all elements satisfy a predicate
//! - [`assert_exists!`] - Ensures at least one element satisfies a predicate
//! - [`assert_unique!`] - Ensures all elements are unique
//! - [`assert_duplicates!`] - Ensures at least one duplicate is present
//! - [`assert_pairwise!`] - Ensures a binary predicate holds for all adjacent pairs

/// Asserts that all elements in the collection satisfy the given predicate.
/// ## Example
/// ```
/// use quantor::assert_forall;
/// 
/// let numbers = vec!(0, 2, 4, 6);
/// 
/// assert_forall!(&numbers, |x| x % 2 == 0);
/// ```
#[macro_export]
macro_rules! assert_forall {
    ($iter:expr, $pred:expr) => {{
        let all = $crate::forall($iter, $pred);
        assert!(all, "assert_forall! failed: not all elements satisfy the predicate.");
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        let all = $crate::forall($iter, $pred);
        assert!(all, $($msg)+);
    }};
}
/// Asserts that at least one element satisfies the predicate.
/// ## Example
/// ```
/// use quantor::assert_exists;
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// 
/// assert_exists!(&numbers, |x| x % 2 == 0);
/// ```
#[macro_export]
macro_rules! assert_exists {
    ($iter:expr, $pred:expr) => {{
        let any = $crate::exists($iter, $pred);
        assert!(any, "assert_exists! failed: no element satisfies the predicate.");
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        let any = $crate::exists($iter, $pred);
        assert!(any, $($msg)+);
    }};
}

/// Asserts that no elements in the iterable satisfy the given predicate.
///
/// ## Example
/// ```
/// use quantor::assert_none;
///
/// let values = vec![1, 3, 5];
/// assert_none!(&values, |x| x % 2 == 0);
/// ```
#[macro_export]
macro_rules! assert_none {
    ($xs:expr, $pred:expr) => {
        if !$crate::none($xs, $pred) {
            panic!("assertion failed: at least one element matched the predicate");
        }
    };
}
/// Asserts that all elements are unique.
/// ## Example
/// ```
/// use quantor::assert_unique;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert_unique!(&numbers);
/// ```
#[macro_export]
macro_rules! assert_unique {
    ($iter:expr) => {{
        let all_unique = $crate::select_duplicates($iter).is_empty();
        assert!(all_unique, "assert_unique! failed: duplicate elements found.");
    }};
    ($iter:expr, $($msg:tt)+) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut all_unique = true;
        for item in $iter {
            if !set.insert(item) {
                all_unique = false;
                break;
            }
        }
        assert!(all_unique, $($msg)+);
    }};
}
/// Asserts that the input contains at least one duplicate.
/// ## Example
/// ```
/// use quantor::assert_duplicates;
/// 
/// let numbers = vec!(0, 0, 1, 2);
/// 
/// assert_duplicates!(&numbers);
/// ```
#[macro_export]
macro_rules! assert_duplicates {
    ($iter:expr) => {{
        let has_duplicates = !$crate::select_duplicates($iter).is_empty();
        assert!(has_duplicates, "assert_duplicates! failed: no duplicates found.");
    }};
    ($iter:expr, $($msg:tt)+) => {{
        let has_duplicates = !$crate::select_duplicates($iter).is_empty();
        assert!(has_duplicates, $($msg)+);
    }};
}
/// Asserts that a binary predicate holds for every adjacent pair in a sequence.
/// ## Example
/// ```
/// use quantor::assert_pairwise;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert_pairwise!(&numbers, |a, b| a < b );
/// ```
#[macro_export]
macro_rules! assert_pairwise {
    ($iter:expr, $pred:expr) => {{
        let any = $crate::pairwise($iter, $pred);
        assert!(any, "assert_exists! failed: no element satisfies the predicate.");
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        let any = $crate::pairwise($iter, $pred);
        assert!(any, $($msg)+);
    }};
}