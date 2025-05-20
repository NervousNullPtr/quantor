/// Evaluates logical quantifiers using mathematical-style syntax over iterables.
///
/// This macro acts as a declarative frontend to the basic quantifier functions in `quantor`,
/// allowing expressive, readable logic similar to mathematics.
///
/// ## Supported Quantifiers
/// - `forall x in &a => predicate`
/// - `exists x in &a => predicate`
/// - `none x in &a => predicate`
/// - `exactly_one x in &a => predicate`
/// - `exactly_n n x in &a => predicate`
/// - `all_equal x in &a`
/// - `pairwise x, y in &a => predicate`
/// - `forallexists x in &a, y in &b => predicate`
/// - `existsforall x in &a, y in &b => predicate`
///
/// See the quantifier functions (e.g. [`forall`](crate::quantifiers::basic::forall)) for behavior.
///
/// ## Examples
/// ```rust
/// use quantor::quantify;
///
/// let xs = vec!(2, 4, 6);
/// assert!(quantify!(forall x in &xs => x % 2 == 0).is_ok());
///
/// let ys = vec!(1, 3, 5);
/// assert!(quantify!(exists x in &ys => *x == 3).is_ok());
/// 
/// let numbers = vec!(1, 1, 1);
/// assert!(quantify!(all_equal x in &numbers).is_ok());
///
/// let a = vec!(1, 2);
/// let b = vec!(3, 4);
/// assert!(quantify!(forallexists x in &a, y in &b => x < y).is_ok());
///
/// let seq = vec!(1, 2, 3);
/// assert!(quantify!(pairwise a, b in &seq => a < b).is_ok());
/// ```
#[macro_export]
macro_rules! quantify {
    // Basic
    (forall $x:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::basic::forall($xs, |$x| $cond)
    };

    (exists $x:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::basic::exists($xs, |$x| $cond)
    };

    (none $x:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::basic::none($xs, |$x| $cond)
    };

    (exactly_one $x:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::basic::exactly_one($xs, |$x| $cond)
    };

    (exactly_n $count:literal $x:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::basic::exactly_n($xs, $count, |$x| $cond)
    };

    (all_equal $x:ident in $xs:expr) => {
        $crate::quantifiers::basic::all_equal($xs)
    };

    (pairwise $a:ident, $b:ident in $xs:expr => $cond:expr) => {
        $crate::quantifiers::structured::pairwise($xs, |$a, $b| $cond)
    };

    // Nested
    (existsforall $a:ident in $as:expr, $b:ident in $bs:expr => $cond:expr) => {
        $crate::quantifiers::nested::existsforall($as, $bs, |$a, $b| $cond)
    };

    (forallexists $a:ident in $as:expr, $b:ident in $bs:expr => $cond:expr) => {
        $crate::quantifiers::nested::forallexists($as, $bs, |$a, $b| $cond)
    };

    ($($t:tt)*) => {
        compile_error!("Invalid syntax in quantify! macro.");
    };
}
