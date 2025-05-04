//! This submodule provides utilities for selecting elements from collections based on logical predicates.
//! 
//! Useful for applications such as enforcing uniquenes or extracting anomalies based on logical rules.

use std::{collections::{HashMap, HashSet}, hash::Hash};

/// Selects elements that satisfy the predicate.
/// 
/// Equivalent to **_{x ∈ self | pred(x)}_**.
/// ## Arguments
/// - `iter` - The collection to filter.
/// - `pred` - The predicate to use for filtering.
/// ## Returns
/// - A `Vec<&T>` containing all matching elements.
/// ## Example
/// ```
/// use quantor::quantifiers::select_where;
/// 
/// let numbers = vec!(0, 1, 2, 3);
/// 
/// assert_eq!(select_where(&numbers, |x| x % 2 == 0), vec!(&0, &2));
/// ```
#[inline]
#[must_use]
pub fn select_where<'a, I, T: 'a, F>(iter: I, pred: F) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> bool,
{
    iter.into_iter().filter(|x| pred(x)).collect()
}

/// Selects elements that satisfy the predicate and returns only unique matches.
/// 
/// Equivalent to **_distinct({x ∈ self | pred(x)})_**.
/// ## Arguments
/// - `iter` - The collection to filter.
/// - `pred` - The predicate to use for filtering.
/// ## Returns
/// - A `Vec<&T>` of matching elements with duplicates removed.
/// ## Example
/// ```
/// use quantor::quantifiers::select_unique;
/// 
/// let numbers = vec!(0, 1, 2, 2, 3);
/// 
/// assert_eq!(select_unique(&numbers, |x| x % 2 == 0), vec!(&0, &2));
/// ```
#[inline]
#[must_use]
pub fn select_unique<'a, I, T, F>(iter: I, pred: F) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Eq + Hash,
    F: Fn(&T) -> bool,
{
    let mut uniques = HashSet::new();
    let mut result  = Vec::new();

    for item in iter {
        if pred(item) {
            if !uniques.contains(item) { // Avoids redundant insertion/check.
                uniques.insert(item);
                result.push(item);
            }
        }
    }

    result
}

/// Selects all duplicate elements in the collection.
/// 
/// Equivalent to **_{x ∈ self | count(x) > 1}_**.
/// ## Arguments
/// - `iter` - The collection to filter.
/// ## Returns
/// - A `Vec<&T>` containing all duplicates found.
/// ## Example
/// ```
/// use quantor::quantifiers::select_duplicates;
/// 
/// let numbers = vec!(0, 1, 2, 2, 3);
/// 
/// assert_eq!(select_duplicates(&numbers), vec!(&2));
/// ```
#[inline]
#[must_use]
pub fn select_duplicates<'a, I, T>(iter: I) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Eq + Hash + Clone,
{
    let mut counts = HashMap::new();
    
    for item in iter {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts.into_iter().filter(|(_, count)| *count > 1).map(|(item, _)| item).collect()
}