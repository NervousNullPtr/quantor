//! Nested quantifiers involving two-level logic.
//!
//! These are helpful for modeling containment, dominance, or existential constraints in rule engines.

/// Checks whether for every element in `a`, there exists at least one element in `b` for which the predicate holds.
/// 
/// Equivalent to **_∀x ∈ a ∃y ∈ b: pred(x, y)_**.
/// ## Arguments
/// * `a` - The source collection (outer quantifier).
/// * `b` - The comparison collection.
/// * `pred` - The binary predicate to check against.
/// ## Returns
/// * `true` if every element in `a` satisfies the condition against some element in `b`.
/// * `false` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::forallexists;
/// 
/// let even = vec!(0, 1, 2, 3);
/// let odd  = vec!(1, 3, 5, 7);
/// 
/// assert!(forallexists(&even, &odd, |x, y| 2 * x + 1 == *y));
/// ```
#[inline]
pub fn forallexists<'a, A: 'a, B: 'a>(
    a: impl IntoIterator<Item = &'a A>,
    b: impl IntoIterator<Item = &'a B>,
    pred: impl Fn(&A, &B) -> bool,
) -> bool {
    let b_collected: Vec<&'a B> = b.into_iter().collect();
    a.into_iter().all(|item_a| {
        for b in &b_collected {
            if pred(item_a, b) {
                return true;
            }
        }
        false
    })
}

/// Checks whether there exists an element in `a` for which all elements in `b` satisfy the predicate.
/// 
/// Equivalent to **_∃x ∈ a ∀y ∈ b: pred(x, y)_**.
/// ## Arguments
/// * `a` - The source collection (outer quantifier).
/// * `b` - The comparison collection.
/// * `pred` - The binary predicate to check against.
/// ## Returns
/// * `true` if at least one element in `a` satisfies the condition against all elements in `b`.
/// * `false` otherwise.
/// ## Example
/// ```
/// use quantor::quantifiers::existsforall;
/// 
/// let smaller = vec!(0, 1, 2, 3);
/// let bigger  = vec!(4, 5, 6, 7);
/// 
/// assert!(existsforall(&smaller, &bigger, |a, b| a < b));
/// ```
#[inline]
pub fn existsforall<'a, A: 'a, B: 'a>(
    a: impl IntoIterator<Item = &'a A>,
    b: impl IntoIterator<Item = &'a B>,
    pred: impl Fn(&A, &B) -> bool,
) -> bool {
    let a_collected: Vec<&'a A> = a.into_iter().collect(); // store all A's
    b.into_iter().any(|item_b| {
        a_collected.iter().all(|item_a| pred(item_a, item_b))
    })
}