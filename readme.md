# quantor
[![Crates.io](https://img.shields.io/crates/v/quantor.svg)](https://crates.io/crates/quantor)
[![Docs.rs](https://docs.rs/quantor/badge.svg)](https://docs.rs/quantor)
[![Downloads](https://img.shields.io/crates/d/quantor.svg)](https://crates.io/crates/quantor)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0+-blue)](https://github.com/nervousnullptr/quantor#msrv)

**Declarative logic for iterators and collections.**  
`quantor` lets you express conditions like `forall`, `exists`, `none`, and `existsforall` directly over data — making filtering, validation, and testing expressive, readable, and idiomatic.

## Why quantor?

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

## Example & Comparison

If you're validating product data from an online shop:

```rust
use quantor::{forall, existsforall, quantify, assert_unique};

#[derive(Debug)]
struct Product {
    id:     u32,
    price:  f64,
    tags:   Vec<&'static str>,
}

#[derive(Debug)]
struct Policy {
    required_tag: &'static str,
}

let products = vec![
    Product { id: 1, price: 19.99, tags: vec!["organic", "new"] },
    Product { id: 2, price: 29.99, tags: vec!["new", "eco"] },
    Product { id: 3, price: 49.99, tags: vec!["premium"] },
];

let policies = vec![
    Policy { required_tag: "new" },
    Policy { required_tag: "eco" },
];

// All quantify!-macro options reside as functions in the quantor crate.

// Each policy must be satisfied by at least one product.
quantify!(forallexists p in &policies, prod in &products =>
    prod.tags.contains(&p.required_tag)
)?;

// At least one product is eco-friendly.
quantify!(exists p in &products => p.tags.contains(&"eco"))?;

// All products are priced.
forall(&products, |p| p.price > 0.0)?;

// IDs are unique.
assert_unique!(&products.iter().map(|p| p.id).collect::<Vec<_>>());
```

This is readable, declarative, and robust – and every check returns a Result with index-level error diagnostics, while the `quantify!` macro makes sure you can write conditions comfortably.

## Installation
Add `quantor` to your `Cargo.toml`:
```
quantor = "0.10"
```
Optional features:
* `method-api` — Enables `.forall()`, `.exists()`, `.select_where()`, etc. on slices and iterators.
* `debug-tools` — Enables `debug_assert_*` and `debug_*` macros for non-panicking diagnostics.

## 📚 Documentation

See [docs.rs](https://docs.rs/quantor) for full API documentation and examples.

## Changelog

See [changelog.md](https://github.com/your-org/your-crate/blob/main/CHANGELOG.md) for version history.

## FAQ

**Why does `forall` return `Result<(), QuantorError>` instead of `bool`?**  
Because failed logical checks are diagnostics, not just `false`. You get indexed errors and rich messages, not just silent failures.

**What’s the difference between `select_where` and `.filter()`?**  
`select_where` is a semantic wrapper — it describes intent. It’s also easier to reuse and test.

## Contributing & Feedback

If `quantor` made your validation or test code cleaner, I'd love to hear about it.

Have ideas? Open an issue or PR!