# quantor
[![Crates.io](https://img.shields.io/crates/v/quantor.svg)](https://crates.io/crates/quantor)
[![Docs.rs](https://docs.rs/quantor/badge.svg)](https://docs.rs/quantor)
[![Downloads](https://img.shields.io/crates/d/quantor.svg)](https://crates.io/crates/quantor)
[![MSRV](https://img.shields.io/badge/MSRV-1.58.1+-blue)](https://github.com/nervousnullptr/quantor#msrv)

**Declarative logic for iterators and collections.**  
`quantor` lets you express conditions like `forall`, `exists`, `none`, and `existsforall` directly over data — making filtering, validation, and testing expressive, readable, and idiomatic.

## Why Quantor?

Rust's iterator methods are powerful, but when you want to write logic that reads like:

- "All elements are even"
- "At least one user is active"
- "Exactly three items matched a condition"

you're often stuck with `.all()`, `.any()`, `.filter().count()`, and some `assert!` noise.

With `quantor`, your code becomes declarative and reflects the logic you care about, not the mechanics.

## Highlights

- **Quantifiers** — `forall`, `exists`, `none`, `exactly_one`, `exactly_n`, `all_equal`, `pairwise`, `forallexists`, and more.
- **Assertions** — Runtime logic assertions like `assert_forall!`, `assert_unique!`, `assert_pairwise!`, with expressive failure output.
- **Predicate-based selection** — Filter, deduplicate, or extract based on logic: `select_where`, `select_unique`, `select_duplicates`.
- **Diagnostics** — Inspect failing indices, collect mismatches, or integrate with fuzzing tools using `QuantorError`.
- **Rust-native, ergonomic API** – Works with any `IntoIterator`, zero dependencies, and optional `.method()` trait extension.

## Example

If you're validating input for a product catalog:

```rust
use quantor::{forall, exists, pairwise, assert_unique};

#[derive(Debug)]
struct Product {
    id:     u32,
    price:  f64,
    active: bool,
}

let products = vec![
    Product { id: 1, price: 19.99, active: true },
    Product { id: 2, price: 29.99, active: true },
    Product { id: 3, price: 0.0,   active: false },
];

// Check that all active products have a price > 0.
forall(&products, |p| !p.active || p.price > 0.0)?;

// Ensure IDs are unique.
assert_unique!(&products.iter().map(|p| p.id).collect::<Vec<_>>());

// Confirm at least one product is available.
exists(&products, |p| p.active)?;
```

This is readable, declarative, and robust – and every check returns a Result with index-level error diagnostics.

## Installation
Add `quantor` to your `Cargo.toml`:
```
quantor = "0.1"
```
Optional features:
* `method-api` — Enables `.forall()`, `.exists()`, `.select_where()`, etc. on slices and iterators.
* `debug-tools` — Enables `debug_assert_*` and `debug_*` macros for non-panicking diagnostics.

## 📚 Documentation

See [docs.rs](https://docs.rs/quantor) for full API documentation and examples.