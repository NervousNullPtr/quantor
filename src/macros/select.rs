/// Extracts values from an iterator using declarative logic-based syntax.
///
/// This macro provides a clean syntax for filtering and selecting values. It expands to corresponding `select_*` functions.
///
/// ## Modes
///
/// - `select!(where x in xs => condition)` — equivalent to `select_where(xs, |x| condition)`
/// - `select!(unique x in xs)` — equivalent to `select_unique(xs)`
/// - `select!(duplicates x in xs)` — equivalent to `select_duplicates(xs)`
///
/// ## Examples
/// ```
/// use quantor::select;
///
/// let xs = vec![1, 2, 3, 4, 4];
///
/// let evens    = select!(where x in &xs => x % 2 == 0);
/// let uniques  = select!(unique x in &xs => *x > 0);
/// let dups     = select!(duplicates x in &xs);
/// ```
#[macro_export]
macro_rules! select {
    (where $x:ident in $xs:expr => $cond:expr) => {
        $crate::select_where($xs, |$x| $cond)
    };

    (unique $x:ident in $xs:expr => $cond:expr) => {
        $crate::select_unique($xs, |$x| $cond)
    };

    (duplicates $x:ident in $xs:expr) => {
        $crate::select_duplicates($xs)
    };

    // Catch all
    ($($t:tt)*) => {
        compile_error!("Invalid syntax in select! macro.");
    };
}