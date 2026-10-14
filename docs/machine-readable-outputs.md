# Machine-Readable Outputs

Text logs are for humans. Automated consumers — CI dashboards, code
scanning, coverage services, and AI evaluators that need to parse, diff,
and reason over results — need something they can read without brittle
log scraping. This page is the index of every structured report RustForge
can emit, how to get it locally, and where CI publishes it.

## Local recipes

All of these write a file next to wherever you run them (the repo root):

| Recipe | Tool | Output | Needs |
|---|---|---|---|
| `just test-json` | cargo-nextest | `nextest.json` — one JSON event per test | `cargo install cargo-nextest --locked` |
| `just coverage-lcov` | cargo-llvm-cov | `lcov.info` — LCOV line coverage | `cargo install cargo-llvm-cov --locked` + `rustup component add llvm-tools-preview` |
| `just clippy-json` | clippy | `clippy.json` — diagnostics as JSON | nothing (stable clippy) |
| `just deny-json` | cargo-deny | `deny.json` — advisory/license/source findings | `cargo install cargo-deny --locked` |
| `just mutants-list-json` | cargo-mutants | `mutants.json` — the mutants cargo-mutants would test (`--json` applies to `--list` only; outcomes come from the `-o` directory — see below) | `cargo install cargo-mutants --locked` |

SARIF (the GitHub code-scanning format, below) is emitted by
`cargo deny --format sarif check` (cargo-deny's `--format` is a global
flag, before the subcommand) and `clippy-sarif` directly; the recipes
above keep to JSON for uniformity.

These are *report* recipes, not gates — with `| tee`, the pipeline's exit
code is `tee`'s, so a recipe like `just clippy-json` reports findings
without failing. The gating versions are `just check`, `just test-all`,
and CI.

## CI artifacts

Every machine-readable report CI produces is uploaded as a build
artifact, downloadable from the workflow run:

| Artifact | Job | Produced by |
|---|---|---|
| `nextest-json` | `nextest` | `cargo nextest run … --message-format libtest-json` |
| `coverage-lcov` | `coverage` | `cargo llvm-cov … --lcov` |
| `clippy-json` | `reports` | `cargo clippy … --message-format json` |
| `deny.sarif` (also uploaded to code scanning) | `deny` | `cargo deny --format sarif check` |
| `clippy.sarif` (also uploaded to code scanning) | `reports` | `clippy-sarif` |

Mutation results are deliberately **not** in CI — mutation testing stays
local and exploratory here (see "Mutation testing is exploratory, not a
gate" in [`best-practices.md`](best-practices.md)).

## What each format is for

### nextest JSON

One JSON object per event (test started / test finished / run summary),
in [libtest-json shape](https://nexte.st/book/output.html). The stable
way to get structured test results — libtest's own `cargo test -- --format
json` exists but is unstable (nightly / `-Zunstable-options`), while
`cargo test --message-format=json` only covers compilation, not test
outcomes.

nextest's value name for the format is `libtest-json`, not `json` (plain
`json` is rejected — the other options are `human` and
`libtest-json-plus`, the latter adding a nextest-specific metadata
subobject). Two gotchas:

- **The format is experimental there** and requires
  `NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1` — without it the run exits 95
  instead of emitting JSON.
- **JSON goes to stdout, human output to stderr**, so `> nextest.json`
  captures pure JSONL while the readable run still shows in the log.
  (`--message-format-version <VERSION>` pins the JSON structure across
  nextest releases for long-term consumers.)

```bash
# Pass/fail per test from a downloaded artifact
jq -r 'select(.type == "test") | "\(.event) \(.name)"' nextest.json
```

### LCOV

The de-facto standard for line coverage: consumed by codecov, coveralls,
and most CI coverage integrations, and diffable between runs
(`lcov -a before.info -a after.info`) without parsing HTML.

### Clippy JSON

Cargo's diagnostic JSON: every clippy warning as a `compiler-message`
object with span, level, and rendered message — feeds editor
integrations and custom reporting alike.

### cargo-deny JSON / SARIF

Supply-chain findings (advisories, license bans, untrusted sources) in
both JSON and SARIF.

### SARIF and code scanning

SARIF (Static Analysis Results Interchange Format) is GitHub code
scanning's native format. The `deny` and `reports` jobs convert their
findings to SARIF and upload them with `github/codeql-action/upload-sarif`,
so findings appear in the repo's **Security → Code scanning** tab next to
their file and line — not just in a CI log.

Two things to know:

- The upload needs `security-events: write`, granted per-job only (the
  workflow's top-level permission stays `contents: read`).
- Upload is `continue-on-error`: it fails on forks and on private repos
  without GitHub Advanced Security, so a fresh copy of this template gets
  no red job it didn't ask for. The SARIF is still generated locally in
  the job either way — see "CI is the source of truth" for the reasoning
  behind this opt-in posture.

### cargo-mutants JSON

Mutation testing stays local and exploratory here (see "Mutation testing
is exploratory, not a gate" in [`best-practices.md`](best-practices.md)),
and cargo-mutants' `--json` flag is narrower than it looks: it applies
**only to `--list`** — it serializes the mutants that *would* be tested
(with their diffs), not the outcomes of a run.

```bash
# Machine-readable list of mutants (what `just mutants-list-json` runs)
cargo mutants --workspace --list --json | tee mutants.json
```

Outcome results *are* machine-readable — just not through `--json`. A run
with `-o <dir>` (what `just mutants-json` does) writes a `mutants.out`
directory containing:

- `outcomes.json` — per-mutant outcomes (`summary` is one of `Success`
  for the baseline, `CaughtMutant`, `MissedMutant`, `Unviable`,
  `Timeout`, each with `log_path`/`diff_path`) plus `total_mutants`,
  `missed`, `caught`, `timeout` counters.
- `mutants.json` — the mutant list, alongside the results.
- `caught.txt` / `missed.txt` / `timeout.txt` / `unviable.txt` — the
  same outcomes as text lists.
- `debug.log` — the full run log.

```bash
# Kill-rate summary from a run's outcomes.json
jq '{total: .total_mutants, missed: .missed, caught: .caught}' mutants.out/outcomes.json
```

`--annotations github` is a third path: findings as GitHub code-review
annotations (works in CI PRs; the options are `none`/`auto`/`github`).

The scoped `just` recipes (`mutants-crate`, `mutants-file`,
`mutants-diff`) keep runs fast — see "Scoping it to what changed" in
[`best-practices.md`](best-practices.md), which also documents the
opt-in soft-reporting CI pattern (informational comment, never a gate).

Treat all of these as informational, like `just mutants` itself — never
a gate.
