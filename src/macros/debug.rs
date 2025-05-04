//! # quantor: Debug Macros
//!
//! This module defines debug-only macros for evaluating logical quantifiers and assertions without
//! affecting release builds. These macros are compiled only when the `debug-tools` feature is enabled.
//!
//! ## Debug Assertion Macros
//! These macros are the debug-only equivalents of the main assertion macros. They panic on failure,
//! but only in debug builds:
//!
//! - [`debug_assert_forall!`] – Asserts all elements satisfy a predicate.
//! - [`debug_assert_exists!`] – Asserts that at least one element satisfies a predicate.
//! - [`debug_assert_none!`] – Asserts that no elements satisfy a predicate.
//! - [`debug_assert_pairwise!`] – Asserts a binary predicate holds for all adjacent pairs.
//! - [`debug_assert_unique!`] – Asserts that all elements are unique.
//! - [`debug_assert_duplicates!`] – Asserts that duplicates exist.
//!
//! ## Debug Inspection Macros
//! These macros do **not panic**; instead, they log failing elements or conditions for inspection,
//! and are useful for fuzzing, diagnostics, and non-blocking testing:
//!
//! - [`debug_forall!`] – Logs elements that violate a predicate.
//! - [`debug_exists!`] – Logs if no elements match.
//! - [`debug_none!`] – Logs which elements unexpectedly match.
//! - [`debug_pairwise!`] – Logs failing adjacent pairs.
//! - [`debug_unique!`] – Logs repeated elements.
//! - [`debug_duplicates!`] – Logs all detected duplicates.

/// Debug-only version of [`assert_forall!`](crate::assert_forall).
/// Panics if any element fails the predicate, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_forall {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::basic::forall($xs, $pred) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_forall! failed: {}", e),
        }
    };
}

/// Logs all elements that fail the predicate.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_forall {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        {
            let failed: Vec<_> = $xs.into_iter().filter(|x| !$pred(x)).collect();
            if !failed.is_empty() {
                println!(
                    "[debug_forall] {} element(s) failed: {:?}",
                    failed.len(),
                    failed
                );
            }
        }
    };
}

/// Debug-only version of [`assert_exists!`](crate::assert_exists).
/// Panics if no element satisfies the predicate, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_exists {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::basic::exists($xs, $pred) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_exists! failed: {}", e),
        }
    };
}

/// Logs a message if no element matches the predicate.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_exists {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        if !$crate::quantifiers::basic::exists($xs, $pred).is_ok() {
            println!("[debug_exists] no matching element found.");
        }
    };
}

/// Debug-only version of [`assert_exactly_n!`](crate::assert_exactly_n).
/// Panics if there are more or less elements which satisfy the predicate than expected, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_exactly_n {
    ($iter:expr, $count:expr, $pred:expr) => {{
        #[cfg(debug_assertions)]
        match $crate::quantifiers::basic::exactly_n($iter, $count, $pred) {
            Ok(()) => {},
            Err(e) => panic!("assert_exactly_n! failed: {}", e),
        }
    }};
}

/// Logs a message if the number of matching elements does not equal the expected count.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_exactly_n {
    ($xs:expr, $count:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        {
            match $crate::quantifiers::basic::exactly_n($xs, $count, $pred) {
                Ok(()) => {},
                Err(e) => {
                    if let Some(found) = e.match_count() {
                        println!(
                            "[debug_exactly_n] expected exactly {}, but found {} matching elements.",
                            $count, found
                        );
                    } else {
                        println!(
                            "[debug_exactly_n] quantifier failed, but count information was not available."
                        );
                    }
                }
            }
        }
    };
}

/// Debug-only version of [`assert_none!`](crate::assert_none).
/// Panics if any element satisfies the predicate, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_none {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::basic::none($xs, $pred) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_none! failed: {}", e),
        }
    };
}

/// Logs all elements that unexpectedly match the predicate.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_none {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        {
            let matched: Vec<_> = $xs.into_iter().filter(|x| $pred(x)).collect();
            if !matched.is_empty() {
                println!(
                    "[debug_none] {} element(s) unexpectedly matched: {:?}",
                    matched.len(),
                    matched
                );
            }
        }
    };
}

/// Debug-only version of [`assert_duplicates!`](crate::assert_duplicates).
/// Panics if no duplicates are found, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_duplicates {
    ($xs:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::selection::assert_duplicates($xs) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_duplicates! failed: {}", e),
        }
    };
}

/// Logs all duplicate elements in the sequence.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_duplicates {
    ($xs:expr) => {
        #[cfg(debug_assertions)]
        {
            use std::collections::HashSet;
            let mut seen = HashSet::new();
            let mut dups = HashSet::new();
            for x in $xs {
                if !seen.insert(x) {
                    dups.insert(x);
                }
            }
            if !dups.is_empty() {
                println!("[debug_duplicates] found duplicates: {:?}", dups);
            }
        }
    };
}

/// Debug-only version of [`assert_unique!`](crate::assert_unique).
/// Panics if any duplicates are found, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_unique {
    ($xs:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::selection::assert_unique($xs) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_unique! failed: {}", e),
        }
    };
}

/// Logs all repeated elements in the sequence.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_unique {
    ($xs:expr) => {
        #[cfg(debug_assertions)]
        {
            use std::collections::HashSet;
            let mut seen = HashSet::new();
            let mut repeated = Vec::new();
            
            for x in $xs {
                if !seen.insert(x) {
                    repeated.push(x);
                }
            }
            if !repeated.is_empty() {
                println!(
                    "[debug_unique] violated: found duplicate(s): {:?}",
                    repeated
                );
            }
        }
    };
}


/// Debug-only version of [`assert_pairwise!`](crate::assert_pairwise).
/// Panics if any adjacent pair fails the predicate, but only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_assert_pairwise {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        match $crate::quantifiers::structured::pairwise($xs, $pred) {
            Ok(()) => {},
            Err(e) => panic!("debug_assert_pairwise! failed: {}", e),
        }
    };
}

/// Logs all adjacent pairs that violate the predicate.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_pairwise {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        {
            let mut failed = Vec::new();
            let mut iter = $xs.into_iter();
            if let Some(mut prev) = iter.next() {
                for curr in iter {
                    if !$pred(prev, curr) {
                        failed.push((prev, curr));
                    }
                    prev = curr;
                }
            }
            if !failed.is_empty() {
                println!(
                    "[debug_pairwise] {} pair(s) failed: {:?}",
                    failed.len(),
                    failed
                );
            }
        }
    };
}
