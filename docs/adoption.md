# Adoption Guide

1. Copy this repository as a template or add the crates under `crates/` into your workspace.
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
