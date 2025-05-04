# quantor
[![Crates.io](https://img.shields.io/crates/v/quantor.svg)](https://crates.io/crates/quantor)
[![Docs.rs](https://docs.rs/quantor/badge.svg)](https://docs.rs/quantor)
[![License](https://img.shields.io/crates/l/quantor)](https://crates.io/crates/quantor)
[![Downloads](https://img.shields.io/crates/d/quantor.svg)](https://crates.io/crates/quantor)
[![MSRV](https://img.shields.io/badge/MSRV-1.61+-blue)](https://github.com/nervousnullptr/quantor#msrv)

**Declarative quantifiers and logical assertions for Rust iterators and collections.**  
`quantor` provides lightweight, expressive tools for validation, filtering, and testing — with zero dependencies.

---

## ✨ Features
`quantor` lets you express logic over data in a way that feels natural and readable:

- 📐 **Quantifiers** — Use familiar constructs like `forall`, `exists`, `none`, and `exactly_one`.
- 🧹 **Selection utilities** — Filter with `select_where`, extract duplicates, check for uniqueness.
- 🧠 **Structured logic** — Run `pairwise` comparisons or validate equality across items.
- 🧪 **Assertions** — Add runtime guarantees with `assert_forall!`, `assert_exists!`, etc.

---

## 🚀 Example

```rust
use quantor::{forall, select_where, assert_forall};

let nums = vec![2, 4, 6];

// Check if all elements are even
assert!(forall(&nums, |x| x % 2 == 0));

// Use the macro version for test-friendly assertions
assert_forall!(&nums, |x| x % 2 == 0);

// Extract matching elements
let evens = select_where(&nums, |x| x % 2 == 0);
assert_eq!(evens, vec![&2, &4, &6]);
```
---

## 📦 Installation
Add this to your `Cargo.toml`:
```
quantor = "0.1"
```
Optional features:
* `method-api` – Enable `.forall()` and other iterator-style methods.
* `debug-tools` – Add debugging macros like `debug_assert_forall!` or `debug_exists!`.

---

## 📚 Documentation

See [docs.rs](https://docs.rs/quantor) for full API documentation and examples.