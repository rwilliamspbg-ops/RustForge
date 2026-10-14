# RustForge → 100% Production-Ready: Execution Plan

This roadmap turns RustForge into a fully production-grade, evaluation-ready
testing foundation. Each item states its impact on the four things the
project cares about most:

- **Default path** — `cargo test --workspace` (no features) must stay fast and
  dependency-light.
- **Modularity** — the seven category crates stay independently adoptable.
- **CI** — narrow, single-purpose jobs; no flakiness; CI and the `justfile`
  stay in sync.
- **MSRV** — stays at 1.75; new work is dev-tooling only (nightly jobs don't
  touch the published MSRV surface).

Status legend: ✅ done, 🚧 in progress, ⬜ not started.

## Audit summary

**Already in place:** seven modular category crates; feature-gated heavy
dependencies; a 13-job CI pipeline (test matrix, nextest, trybuild, loom,
fuzz-build, coverage, udeps, msrv, hack, minimal-versions, deny, docs, and
adoption); Loom as a first-class citizen (`semantic-tests` `loom` feature +
dedicated CI job); genuine `#![no_std]` verification; MSRV and
minimal-versions jobs; supply-chain checks on both workspaces; mutation
testing available locally via `just mutants`.

**The real work (from the external review, cross-checked against the repo):**

| Gap | Phase |
|---|---|
| Adoption path never verified in CI | Phase 1 (✅ 1.1) |
| Miri (UB detection) — absent entirely | Phase 1 (✅ 1.2) |
| Machine-readable outputs (JSON/SARIF) | Phase 1 (✅ 1.3) |
| Release hygiene (version, changelog, doc sync) | Phase 1 (✅ 1.4) |
| Code-under-test evaluation harness | Phase 2 (✅ 2.1) |
| Differential-testing helpers as code | Phase 2 (✅ 2.2) |
| Mutation-testing polish (scoping, JSON, soft reporting) | Phase 2 (2.3) |
| const/macro test examples | Phase 3 |
| Domain starter examples (async service, parser, CLI) | Phase 3 |
| Quality signals (cargo-bloat, binary size, compile times) | Phase 3 |
| AI test-generation aids doc | Phase 3 |
| Minimal-adoption and test-promotion guidance | Phase 3 |

## Phase 1 — P0: Release blockers (target: 1.0.0)

### 1.1 Adoption-path verification in CI ✅

The template's core promise is "copy `crates/` + adjust workspace" — verified
in CI, not just documented. **Shipped** in this changeset:

- New stable-only `adoption` job in `.github/workflows/ci.yml` with two legs:
  1. `cargo generate --path .` (via `cargo-generate`) into a temp dir, then
     `cargo test --workspace` in the output — the "fresh git history"
     adoption path.
  2. A synthetic host workspace (own root package, RustForge's `crates/`
     copied in, `[workspace.package]`/`[workspace.lints.rust]` tables
     present) — the "existing workspace" path, including the one real gotcha
     from `docs/adoption.md`.
- `scripts/adoption-check.sh` mirrors both legs locally; `just adopt-test`
  calls it (skips the `cargo-generate` leg if the tool isn't installed,
  matching the `full-ci` deny-skip pattern).
- **Impact:** default path untouched; +1 stable-only CI job (~3 min,
  dominated by the `cargo install cargo-generate` step); no MSRV effect.

### 1.2 Miri as a first-class citizen ✅

Undefined behavior that happens to work on your machine is invisible to
native execution — tests, and even fuzzing, only catch UB that
misbehaves on the host. Miri *interprets* the program and detects it
directly. **Shipped** in this changeset:

- `just miri`: `cargo +nightly miri test` scoped to default features (Miri
  can't run tokio/loom/criterion/trybuild — documented).
- Nightly-only `miri` CI job: `cargo +nightly miri test -p core-tests -p
  semantic-tests` (semantic without `async`/`loom` features).
- Docs section in `docs/best-practices.md`: what Miri catches that tests and
  fuzzing miss (UB in unsafe code, alignment, data races under the Miri
  scheduler).
- **Impact:** zero runtime dependencies; nightly toolchain only; no
  default-path cost.

### 1.3 Structured, machine-readable outputs ✅

Text logs are for humans; automated consumers — CI dashboards, code
scanning, coverage services, AI evaluators — need something parseable.
**Shipped** in this changeset:

New doc `docs/machine-readable-outputs.md` + `just` recipes; pure tooling:

- `just test-json`: `cargo nextest run --workspace --features … --message-format libtest-json`
- `just coverage-lcov`: `cargo llvm-cov --lcov --output-path lcov.info` (+ `--json`)
- `just mutants-list-json` / `just mutants-json`: `cargo mutants --list
  --json` for the mutant list; outcomes via `-o mutants.out`
  (`outcomes.json`) — scoped per 2.3
- `just clippy-json`: `cargo clippy … --message-format json`;
  `just deny-json`: `cargo deny --format json check` (`--format` is
  global in cargo-deny, before the subcommand)
- CI: upload JSON artifacts from the test/coverage jobs; SARIF upload for
  clippy + deny via `github/codeql-action/upload-sarif` (opt-in,
  `continue-on-error`).
- **Impact:** CI artifacts only; default path untouched.

### 1.4 Release hygiene ✅

**Shipped** in this changeset:

- Cut `0.2.0-alpha` → `1.0.0` — the CHANGELOG's `Unreleased` section is
  now `[1.0.0] - 2026-10-14` with a fresh empty `Unreleased` above it.
- README / `ci/README.md` / CONTRIBUTING synced for every new job and
  recipe (standing repo convention — CI is the source of truth).
- Removed the placeholder root `tests/README.md` (and the `tests/` line in
  README's workspace layout) — it documented a "root package" flow that
  can't exist in a virtual workspace.

**Phase 1 exit criteria:** `cargo generate` output passes its own tests in
CI; Miri job green; JSON artifacts downloadable from a PR; 1.0.0 cut with a
complete CHANGELOG. — **All met** (CI runs on the PR).

## Phase 2 — P1: Evaluation / AI-readiness

### 2.1 Code-under-test evaluation harness ✅

New **non-member** example crate `examples/eval-harness/` (kept out of
workspace `members` — an empty `[workspace]` table detaches it, the
`fuzz/` pattern — preserving default `cargo test --workspace` speed):

- **Shipped** in this changeset, alongside `examples/sample-candidate/`:
- Fixed trait interface (`Candidate`) + golden tests + property tests + a
  trybuild negative case, all written against the trait only.
- Adapter pattern: `FnCandidate` wraps a plain `fn(&str) -> Option<u32>`
  as a `Candidate` — the thin adapter layer for candidates whose real API
  doesn't match the trait shape, with no changes to the code under test.
- `just eval <path-to-candidate>`: generates a throwaway workspace, wires
  the candidate in via path dependency, runs the full suite (or a
  `--features` subset).
- **Impact:** zero default-path deps; opt-in by copying the example;
  modularity preserved by non-membership.

### 2.2 Differential-testing helpers (code, not just docs) ✅

**Shipped** in this changeset:

- `core-tests` module `differential`:
  - `assert_impls_agree(a, b, inputs)` — two callables, same inputs, equal
    outputs/errors.
  - Property-based variant reusing `fuzz-tests`' existing `proptest` feature
    (don't duplicate the dependency).
- Worked examples ship as real tests, not just docs: numeric
  (closed-form sum vs loop) and primality (naive trial division vs
  sqrt-bounded optimization) — plus the same two pairs over generated
  inputs in the `fuzz` feature.
- **Impact:** one small module; proptest variant stays feature-gated.

### 2.3 Mutation-testing polish (stays exploratory — hard rule) ⬜

- `just mutants-crate <crate>` / `just mutants-file <path>` via cargo-mutants'
  path/package filters; `--json` output (ties into 1.3).
- Docs: a documented *soft* reporting pattern (informational workflow comment
  with kill rate, `continue-on-error`) — **never a merge gate**.
- **Impact:** local tooling; one informational CI job at most.

**Phase 2 exit criteria:** an external implementation can be evaluated
against the suite with one command; two implementations can be diff-tested
via helper; mutants scoping is scriptable.

## Phase 3 — P2: Breadth & niceties

- **const / macro testing examples:** `core-tests` const-eval example + a
  `macro_rules!` trybuild pair, wired into existing feature lists (no new CI
  job — the `no_std_check` precedent).
- **Domain starter examples** (async service, parser, small CLI) as
  non-member example directories with standalone-workspace instructions —
  same containment strategy as the eval harness.
- **Quality signals (opt-in):** `just bloat` (cargo-bloat), binary-size
  tracking, `cargo build --timings` artifacts — docs/local recipes only, no
  CI gates.
- **Docs:** "minimal adoption" path (syntax + semantic + a few properties =
  smallest useful signal) and "promoting template tests into the real
  library's own tests" guidance in `docs/adding-tests.md`.
- **AI test-generation aids doc:** signature→property recipes, fill-in
  snapshot/trybuild templates, category/fixture recipes for LLM-assisted use.

**Phase 3 exit criteria:** every nice-to-have from the external review is
either implemented behind opt-in flags or documented as a deliberate
non-goal.

## Non-goals (deliberately preserved)

- Mutation kill-rate thresholds as a hard CI gate — stays informational.
- Any new *default* dependency; `cargo test --workspace` stays fast.
- Raising MSRV above 1.75.
- Coupling category crates; new harnesses stay non-member examples.

## Sequencing

Phase 1 items are independent and parallelizable. Phase 2.1 (eval harness) is
the largest single item and depends on nothing.

```
1.1 adoption CI ─┐
1.2 Miri         ├─→ 1.4 release hygiene (cut 1.0.0)
1.3 JSON/SARIF  ─┘         │
                            ▼
              2.1 eval harness → 2.2 differential helpers → 2.3 mutants polish → Phase 3
```
