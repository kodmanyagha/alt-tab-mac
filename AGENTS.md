# AGENTS.md

Standards for the alt-tab-mac repository, whether the work is done by a human or an AI agent.

## Code rules

- **NEVER USE `unwrap()` OR `expect()` ANYWHERE EXCEPT DURING APPLICATION STARTUP. PROPAGATE ERRORS WITH `?`, `ok_or`/`ok_or_else` OR `map_err`; RECOVER POISONED LOCKS WITH `unwrap_or_else(PoisonError::into_inner)`. TEST CODE (`#[cfg(test)]`, `tests/`) IS THE ONLY OTHER EXCEPTION.**
- To turn an `Option<T>` into a `Result<T>` with a fixed error, use `ok_or`; when the error has to be built, use `ok_or_else`.
- Keep functions short and at one level of abstraction. A function either orchestrates (calls named steps in order, so it reads like a table of contents) or does one concrete step, never both. When a function has distinct stages, move each stage into its own well-named function so the caller reads as a list of what happens.
- Write function documentation only; never write any other comment. Use good names for functions, structs, enums, enum fields and variables instead. The only exception is the `// SAFETY:` comment required above an `unsafe` block.

## Rust style

Adapted from the [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html); rules specific to Chromium, Android or Fuchsia are left out. Where it conflicts with "Code rules" above, "Code rules" wins.

### General

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/) and [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) unless a rule here says otherwise.
- Favor readability and maintainability over speed of writing, and stay consistent with the wider Rust ecosystem.
- This is application code: treat the API Guidelines as strong recommendations, favor simplicity and do not generalize before it is needed.
- Formatting is done by `rustfmt`; do not argue about formatting in review. Use `#[rustfmt::skip]` only for deliberate manual alignment or generated code, never for personal preference.

### Code organization

- Name a file after the main item it holds: `OrderBook` lives in `order_book.rs`. Module files are `snake_case`, types `PascalCase`, constants `SCREAMING_SNAKE_CASE`.
- `mod.rs` files and other non-leaf modules contain only module declarations (`pub mod ...`) and re-exports, no logic.
- Avoid glob imports. Allowed: `use super::*` in an inline `mod tests`, and `pub use submodule::*` in a non-leaf module. Do not define preludes.
- Do not mix module paths inside one brace group: write `use foo::Foo;` and `use foo::bar::Bar;`, not `use foo::{Foo, bar::Bar};`. Several items from the same module in one brace group are fine: `use foo::{Bar, Foo};`.
- Import a trait inside the function that uses it, unless it is used throughout the file.
- Call free functions through their module: `mem::take(&mut x)`, not a bare `take(&mut x)`.
- Import types and use them unqualified: `Regex`, not `regex::Regex`. Qualify or rename with `as` when the name is ambiguous, for example `io::Error` or `use std::fmt::Result as FmtResult`.

### Ownership and borrowing

- Prefer unique ownership and borrowing. Use `Rc`/`Arc` for state that is really shared, such as state shared across threads, not to avoid thinking about lifetimes.
- Prefer borrowing over deep copies. Clone only when an owned copy is really needed, and keep clones out of hot paths. Cloning an `Rc`/`Arc` is fine.

### Function inputs and outputs

- Take the most flexible reference for read-only data: `&str` instead of `&String`, `&[T]` instead of `&Vec<T>`, `&T` instead of `&Box<T>`, `Option<&T>` instead of `&Option<T>`.
- Returning `Vec<T>` and accepting slices is fine in application code. Return or accept iterators when the caller only iterates and it keeps the code simple.
- Avoid `impl Into<T>` parameters in application code; take the concrete type.
- Use an enum instead of a `bool` parameter when the meaning of `true`/`false` is not obvious at the call site.
- Errors: return `Option` for a simple, obvious "not found" case and `Result` when the caller needs to know why it failed. Panics follow the `unwrap`/`expect` rule in "Code rules".

### Types

- Use a newtype instead of a type alias when the value has its own meaning, needs its own methods, may change representation, or must not be mixed up with another value of the same underlying type (for example `UserId(u64)` versus `OrderId(u64)`).
- Use structs with named fields instead of tuples, except for single-field newtypes and very local use.

### Traits

- Implement standard traits instead of custom methods: `FromStr` instead of a free `from_str`, `From` instead of `to_foo` conversions where it fits. Pair an important `From` impl with an inherent constructor (`new`, `from_foo`).
- Prefer `#[derive(...)]` over manual impls.
- Define a trait only for generic code, for `impl Trait`/`dyn Trait` parameters or returns, or as an extension trait. Do not create traits just to group methods; use inherent `impl` blocks.
- Name extension traits on foreign types with the `Ext` suffix.
- Prefer generics over `dyn Trait` unless code size or heterogeneous collections require `dyn`.
- Derive `Clone` for most types except those owning external resources; `Copy` when it will stay valid; `Default` when there is a `new()` or a natural default; `Hash` and `Eq` for map keys, plus `Ord` when ordering is needed. Prefer `PartialEq`/`PartialOrd` unless the stronger trait is needed.
- Options/config structs with public fields implement `Default`.

### Concurrency

- Never read through a poisoned lock as if nothing happened; recover it only as "Code rules" allows, and only when the protected data stays valid.

### Macros

- Prefer, in order: plain Rust, `macro_rules!`, build-time code generation, `proc_macro`. Macros hurt IDE support and code search, so add one only when it removes real repetition.

### Naming

- Follow standard library conventions: `as_` (cheap borrow), `to_` (costly conversion), `into_` (consumes `self`).
- Constructors: `new()`, `new_with_foo()`, `try_new()` when construction can fail, `from_foo()` alongside a `From` impl.
- Accessors: `foo()` getter, `foo_mut()` mutable getter, `set_foo()` setter, `take_foo()` / `replace_foo()` when the old value is returned.
- If a type has `len()`, give it `is_empty()` too.
- Give lifetimes descriptive names except in simple cases (`'_`, or one or two single-letter lifetimes for a plain input/output relation). Generic type parameters may be short (`T`, `U`) when unbounded.
- Avoid abbreviations unless they are common and clear to someone new to the code (`IpAddr`, `Id`, `Ms`).

### Documentation

- Document public functions, types and modules with `///` (items) and `//!` (module or crate, at the top of the file), written in Markdown with proper capitalization and punctuation.
- Document panic conditions and the invariants of any `unsafe` code.
- Do not restate the obvious or explain language basics. Leaving out a doc that would only repeat the name is better than writing it.
- Doc comments for enum variants are optional when the variant name is self-explanatory.

### Other language features

- Avoid `unsafe`. If it is unavoidable, wrap it in a small safe abstraction and put a `// SAFETY:` comment above each `unsafe` block explaining why it is sound. If performance is the reason, include benchmark numbers.
- Do not use unstable (nightly) features.
- Use operator overloading only when the operator has a clear mathematical or logical meaning.
- Mark functions whose result must not be ignored with `#[must_use]`, and the same for types such as error types and lazy computations.
- Make a function `const fn` only if it is meant to stay `const`.
- No code that runs before `main()` (for example the `ctor` crate).

### Testing

- **Unit tests are written inside the file.** Do not open a separate `tests.rs` or `src/tests/` folder; append a `#[cfg(test)] mod tests { ... }` block at the end of the file. Any new module containing logic gets its tests in this same in-file style.
- **Integration tests go in a `tests/` folder** of the crate. They exercise the crate's public API from the outside, as a consumer would.
- **`examples/` is not used for tests** — it holds manual/demonstration runnable scenarios and is not run by `cargo test`.
- Test naming: `#[test] fn <scenario>_<expected_outcome>()`, descriptive (e.g. `rejects_when_qty_exceeds_available_balance`).

### Tooling and logging

- New code must not introduce clippy warnings (`cargo clippy --all-targets` must stay clean for the code you touch).
- Logging is done via `tracing` (`tracing::info!`, `tracing::error!`, using structured fields — e.g. `window_count = windows.len()`). `println!`/`eprintln!` are only acceptable in non-production code such as `examples/` and command-line tools.

### Implementation

- Do not overuse type inference: add an explicit type where the type is not obvious, especially after `.into()`, `.parse()` and `.collect()` with a complex result. Prefer a type annotation on the binding over a turbofish.
- Use pattern matching (`if let`, `let ... else`, `match`) instead of checking and then unwrapping.
- In `match` over an enum, list every variant you handle explicitly; use a wildcard only for variants you really do not care about, and `if let` when only one or two variants matter.
- Give `assert!` a message that explains the invariant or the values involved, not one that repeats the condition.
- Keep iterator chains short and readable. Do not build a chain inside a `for` header, and do not use `for_each`; use a `for` loop.
- Do not use `catch_unwind`.
- Build strings with `format!`/`write!`, not `+` concatenation or `str::replace` placeholders.
