# Adoption Guide

1. Copy this repository as a template or add the crates under `crates/` into
   your workspace. If you have [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/)
   installed, `cargo generate --git <this-repo-url>` does the same copy with
   a fresh git history in one command — nothing gets renamed or templated
   (see `cargo-generate.toml`'s comment for why), it's purely a convenience
   over manually cloning and deleting `.git`.
2. Keep `core-tests` as the shared helper layer.
3. Enable only the categories you need first (`syntax`, `semantic`, `integration`).
4. Run `cargo test --workspace` and incrementally add categories (`perf`, `fuzz`, `edge`).
5. Reach for the dependency-pulling feature flags (`async`, `perf`, `fuzz`) only
   when a category needs them — see the feature table in the root `README.md`.
   Running `cargo test --workspace --all-features` locally (or `scripts/check.sh`)
   exercises everything at once, matching what CI does.
6. When a category needs real property-based fuzzing beyond in-process
   `proptest` checks, build on the `fuzz/` scaffold with `cargo-fuzz`
   (nightly-only, detached from the main workspace).
7. Use `scripts/coverage.sh` (needs `cargo-llvm-cov`) to spot untested paths
   before adding new categories or expanding existing ones.

## Migrating an existing project into RustForge's structure

The steps above assume you're bringing RustForge's crates *into* your
workspace. This section covers the other direction: you already have tests —
probably `#[cfg(test)] mod tests { ... }` blocks scattered across your
source files — and want to reorganize them into RustForge's category
crates instead of writing everything from scratch.

1. **Don't migrate everything at once.** Add RustForge's crates alongside
   your existing `#[cfg(test)]` modules (see "Common Pitfalls" below for the
   one likely snag) and leave the old tests running. `cargo test --workspace`
   runs both; there's no conflict. Migrate incrementally, category by
   category, as you touch each area of code.
2. **Sort your existing tests by what they're actually checking, not where
   they live today.** A `#[cfg(test)]` module next to a parser function and
   one next to a business-logic function are testing fundamentally different
   things even though they look identical structurally. Use the "Where does
   my test go?" table (or flowchart) in
   [`docs/adding-tests.md`](adding-tests.md) to sort each existing test
   function into a category — most plain `#[test]` functions map to
   `syntax-tests`, `semantic-tests`, or `edge-cases` depending on what
   they're actually asserting, not to a single catch-all category.
3. **Move shared setup/helper code to `core-tests` first.** If your existing
   tests have hand-rolled fixture builders or repeated setup code, that's
   the first thing to port — before moving the tests that use it — so the
   categories you migrate afterward can depend on it instead of duplicating
   it. See "Reusing fixtures" in [`docs/adding-tests.md`](adding-tests.md#reusing-fixtures).
4. **Only add property/fuzz/snapshot/benchmark tests where they earn their
   keep.** A plain `#[test]` that already passes doesn't need to become a
   `proptest` property test just because `fuzz-tests` exists — those shapes
   are for cases where a handful of hand-picked inputs genuinely aren't
   enough (see "The five test shapes" in `docs/adding-tests.md`). Migrating
   is about relocating and categorizing what you have, not rewriting it into
   every available test shape.
5. **Delete the old module once its replacement is in and passing, not
   before.** Keep both versions running side by side for at least one CI
   cycle so a categorization mistake shows up as a duplicate failure, not a
   silent gap in coverage.

## Common Pitfalls

These are things that actually broke while testing the "copy `crates/`
into an existing workspace" flow end-to-end (a fresh workspace with its
own root package, RustForge's `crates/` copied in, `members` merged) —
not a guess at what might go wrong.

### `workspace.package.edition was not defined`

The single most likely first error. Every crate here uses
`edition.workspace = true` / `license.workspace = true` /
`rust-version.workspace = true` (see `docs/best-practices.md` for why —
it's what makes those fields *mean* something instead of being unused
metadata). If your existing root `Cargo.toml` doesn't already have a
`[workspace.package]` table, `cargo test --workspace` fails immediately
with:

```text
error: failed to parse manifest at `.../crates/core-tests/Cargo.toml`

Caused by:
  error inheriting `edition` from workspace root manifest's `workspace.package.edition`

Caused by:
  `workspace.package.edition` was not defined
```

Fix: add this to your root `Cargo.toml` (adjust to your own project's
edition/MSRV if you already have one — RustForge's crates just need
*some* value present, and `rust-version` should be `1.75` or higher to
keep the MSRV pins on `proptest`/`trybuild` meaningful):

```toml
[workspace.package]
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"
```

### Optional dotfiles aren't copied by the Quickstart, and that's fine

`rustfmt.toml`, `clippy.toml`, `deny.toml`, `.gitattributes`, and
`.editorconfig` aren't mentioned in the Quickstart's "copy `Cargo.toml` +
`crates/`" step — tests pass without them. Copy whichever ones you want;
each is independently useful (see their comments) and none depend on the
others. `deny.toml` in particular is only useful once you also copy (or
write your own) `.github/workflows/ci.yml` job calling `cargo-deny`.

### Your own crates coexist fine — no naming or feature collisions observed

Merging RustForge's 7 crates alongside an existing root package (in
testing: a plain binary crate) and running `cargo test --workspace` picked
up and ran every crate's tests, including doc-tests, without any manual
per-crate configuration beyond the `[workspace.package]` fix above.
