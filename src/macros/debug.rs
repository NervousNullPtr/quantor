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
        $crate::assert_forall!($xs, $pred);
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
            let mut failed = Vec::new();
            for x in $xs {
                if !$pred(x) {
                    failed.push(x);
                }
            }
            if !failed.is_empty() {
                println!(
                    "[debug_forall] failed for {} element(s): {:?}",
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
        $crate::assert_exists!($xs, $pred);
    };
}

/// Logs a message if no element matches the predicate.
/// Does not panic. Active only in debug builds.
#[cfg(feature = "debug-tools")]
#[macro_export]
macro_rules! debug_exists {
    ($xs:expr, $pred:expr) => {
        #[cfg(debug_assertions)]
        {
            if !$crate::exists($xs, $pred) {
                println!("[debug_exists] no matching element found.");
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
        $crate::assert_none!($xs, $pred);
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
            let mut matched = Vec::new();
            for x in $xs {
                if $pred(x) {
                    matched.push(x);
                }
            }
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
        $crate::assert_duplicates!($xs);
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
        $crate::assert_unique!($xs);
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
            let mut duplicates = Vec::new();
            for x in $xs {
                if !seen.insert(x) {
                    duplicates.push(x);
                }
            }
            if !duplicates.is_empty() {
                println!(
                    "[debug_unique] violated: found duplicate(s): {:?}",
                    duplicates
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
        $crate::assert_pairwise!($xs, $pred);
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
                    "[debug_pairwise] predicate failed on {} pair(s): {:?}",
                    failed.len(),
                    failed
                );
            }
        }
    };
}