# Best Practices

Conventions this template follows, and why — so you can keep following
them (or make a deliberate, informed decision not to) as you extend it.

## Keep the default build dependency-light

`cargo test --workspace` with no flags should stay fast and pull in
nothing beyond the standard library and this template's own crates. Every
category that needs real ecosystem tooling (Tokio, Criterion, proptest,
trybuild) puts it behind a Cargo feature — see the table in the root
[`README.md`](../README.md#feature-flags). This means:

- New adopters get a fast, no-surprises first `cargo test` run.
- CI's default matrix stays cheap; heavier checks (fuzzing, benchmarks,
  compile-fail tests) run as their own dedicated jobs.
- `[dev-dependencies]` **cannot** be made optional (a Cargo limitation) —
  if you add one, know that it's resolved unconditionally by any `cargo
  test`/`cargo bench` invocation for that crate, feature flag or not. See
  the comment above `criterion` in `crates/performance-tests/Cargo.toml`
  for a real example of this biting us (a transitive dependency's jump to
  requiring `edition2024` broke the `msrv` CI job even though the `perf`
  feature was never enabled).

## Pin, don't chase, MSRV-sensitive dependencies

The workspace declares `rust-version = "1.75"` for the *default* build.
When an optional feature's dependency releases a version that needs a
newer compiler, either:

1. **Pin to the newest MSRV-compatible release** if one exists — see how
   `proptest` (`~1.8`) and `trybuild` (`=1.0.111`) are pinned in
   `crates/fuzz-tests/Cargo.toml` / `crates/syntax-tests/Cargo.toml`, each
   with a comment explaining the ceiling. Use `=` (exact) rather than `~`
   when multiple patch releases exist within what looks like a safe range
   — `~1.0.111` still floats across 1.0.112–1.0.116, which turned out to
   already exceed the MSRV.
2. **Accept the drift and document it** if pinning isn't practical (e.g. a
   dependency several levels deep in a large transitive graph, like
   `criterion`'s `clap`/`regex`/`plotters` chain) — see the "MSRV" section
   of the root README for how that tradeoff is written up.

Either way, tell Dependabot: add an `ignore` rule in
`.github/dependabot.yml` for anything you pin (see the `proptest`/
`trybuild` entries there). Dependabot doesn't know *why* a version is
pinned — without the ignore rule, it will cheerfully propose bumping
straight past it, and if merged without review, silently break the MSRV
promise. This happened once already; see the CHANGELOG.

## Forbid `unsafe` unless a category specifically needs it

The workspace's `[lints.rust]` table in the root `Cargo.toml` sets
`unsafe_code = "forbid"`, inherited by every crate via `[lints] workspace =
true` — one declaration instead of a repeated `#![forbid(unsafe_code)]` per
crate. If you're adding a category that genuinely needs `unsafe` (e.g. an
FFI or `no_std`-focused category), override it locally with an explicit,
documented `#![allow(unsafe_code)]` in that crate — don't change the
workspace default for everyone else.

## Document every public item

The same workspace `[lints]` table sets `missing_docs = "warn"`, and CI runs
clippy with `-D warnings`, which turns that into a hard error. This is
enforced, not aspirational — a PR that adds an undocumented `pub fn` fails
CI. Doc comments here should explain *what a reader can't get from the
signature*: invariants, panics, feature-flag interactions — not restate the
function name in prose.

## Prefer real fixtures over mocks

`core-tests`'s `UserFixture` is a plain struct with builder methods, not a
mock framework. Keep it that way: fixtures here exist to make tests
readable and consistent across categories, not to simulate behavior. If a
category genuinely needs mocking (e.g. testing retry logic against a
flaky dependency), that's a local concern for that category, not something
to push into the shared `core-tests` layer.

## Match the test to the failure mode

- Reaching for a runtime `Result`/panic check when the real question is
  "does this compile"? Use `trybuild` (`compile-fail` feature), not a
  unit test — see [`docs/adding-tests.md`](adding-tests.md).
- Testing a handful of hand-picked inputs when the real property should
  hold for *all* inputs? Use `proptest` (`fuzz` feature).
- Testing correctness of a hot path but not its speed? A plain unit test
  is enough — save Criterion for when you actually need a regression
  guard, not by default for every function.

## Mutation testing is exploratory, not a gate

`just mutants` (wrapping [`cargo-mutants`](https://mutants.rs/)) runs a
local, informational check of how much your test suite would actually
notice if the code under test broke — the same category of signal as the
`coverage` CI job, and deliberately kept to the same scope: nothing wires it
into CI, and there's no required mutation-kill-rate threshold. A hard gate
here produces tests written to kill mutants rather than to verify real
behavior, the same failure mode a hard coverage floor produces (see "Keep CI
jobs narrowly scoped" below and the `coverage` job's comment in
`.github/workflows/ci.yml`). Run it periodically against a category you're
about to refactor, not as a required check.

## `cargo-semver-checks` doesn't apply here

Every crate in this workspace sets `publish = false` — RustForge is meant to
be copied or vendored into an adopter's own workspace (see
[`docs/adoption.md`](adoption.md)), not consumed via `cargo add` against a
version published to crates.io. `cargo-semver-checks` exists to catch
breaking changes in a *published* crate's public API between releases; that
contract doesn't exist here, so adding it wouldn't catch anything meaningful
today. If this project's distribution model ever shifts toward something
adopters pull updates from (e.g. a `cargo-generate` template with versioned
releases) rather than a one-time copy, this is worth revisiting then.

## Keep CI jobs narrowly scoped

Prefer several small, single-purpose CI jobs over one job doing
everything with `--all-features`. This template's `test` job deliberately
excludes `syntax-tests`' `compile-fail` feature (isolated to its own
stable-only `trybuild` job) because trybuild asserts on `rustc`'s exact
diagnostic text, which can drift between stable/beta/nightly — see
[`ci/README.md`](../ci/README.md). A job that mixes concerns like that
either flakes for reasons unrelated to your change, or quietly stops
testing what it claims to.
