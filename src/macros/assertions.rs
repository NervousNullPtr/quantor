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
        match $crate::quantifiers::basic::forall($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_forall! failed: {}", e),
        }
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        match $crate::quantifiers::basic::forall($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_forall! failed: {} — {}", format_args!($($msg)+), e),
        }
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
        match $crate::quantifiers::basic::exists($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_exists! failed: {}", e),
        }
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        match $crate::quantifiers::basic::exists($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_exists! failed: {} — {}", format_args!($($msg)+), e),
        }
    }};
}

/// Asserts that at least one element satisfies the predicate.
/// ## Example
/// ```
/// use quantor::assert_exactly_n;
/// 
/// let numbers = vec!(0, 1, 3, 5);
/// 
/// assert_exactly_n!(&numbers, 1, |x| x % 2 == 0);
/// ```
#[macro_export]
macro_rules! assert_exactly_n {
    ($iter:expr, $count:expr, $pred:expr) => {{
        match $crate::quantifiers::basic::exactly_n($iter, $count, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_exactly_n! failed: {}", e),
        }
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
    ($iter:expr, $pred:expr) => {{
        match $crate::quantifiers::basic::none($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_none! failed: {}", e),
        }
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        match $crate::quantifiers::basic::none($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_none! failed: {} — {}", format_args!($($msg)+), e),
        }
    }};
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
        match $crate::quantifiers::structured::pairwise($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_pairwise! failed: {}", e),
        }
    }};
    ($iter:expr, $pred:expr, $($msg:tt)+) => {{
        match $crate::quantifiers::structured::pairwise($iter, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_pairwise! failed: {} — {}", format_args!($($msg)+), e),
        }
    }};
}