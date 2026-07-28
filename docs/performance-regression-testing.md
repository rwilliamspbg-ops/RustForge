# Performance Regression Testing

Two tools for two different questions, both already in this template:

- **"Is this still correct and not *catastrophically* slower?"** — the
  `#[ignore]`d wall-clock guards next to the unit tests in
  `crates/performance-tests/src/lib.rs` (e.g. `sum_large_input_is_reasonably_fast`).
  No extra dependencies, runs under plain `cargo test -- --ignored`. Good
  as a coarse tripwire, bad at answering "is a 3% regression real or noise?"
- **"Did this change actually make it slower, with statistical
  confidence?"** — Criterion's built-in baseline comparison, covered below.
  This is what to reach for when reviewing a PR that touches hot-path code.

## Establishing a baseline

```bash
cargo bench -p performance-tests --features perf --bench sum_bench --bench collection_ops_bench -- --save-baseline main
```

Naming the bench targets explicitly matters: plain `cargo bench` (and even
`cargo bench --benches`) also invokes the crate's own unit-test binary,
because Cargo's `bench` manifest field defaults to `true` for the library
target too — `--benches` selects "targets with `bench = true`", which
includes it. That binary doesn't understand Criterion's flags, so
`--save-baseline`/`--baseline` fail on it with `Unrecognized option`.
`--bench <name>` (repeated per target) is the precise fix. (`just
bench-baseline`/`just bench-compare` below already do this for you.)

Run this on the commit you want to compare against (typically before your
change, or on `main`). Criterion stores the results under
`target/criterion/` (gitignored — this is local, disposable state, not
something to commit).

## Comparing against it

Make your change, then:

```bash
cargo bench -p performance-tests --features perf --bench sum_bench --bench collection_ops_bench -- --baseline main
```

Criterion prints a `change: [...]` line with a confidence interval and a
p-value per benchmark, e.g.:

```text
sum/1000                time:   [525.49 µs 526.05 µs 526.63 µs]
                        change: [+602869% +604072% +605243%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

`p < 0.05` (Criterion's default significance threshold) means the change
is unlikely to be noise. This was verified against a real, deliberately
introduced regression (a `thread::sleep` added to `sum` and reverted
immediately after) — Criterion correctly flagged "Performance has
regressed" with `p = 0.00` on all three input sizes.

## `--quick` gives false negatives — don't use it for real comparisons

Criterion's `--quick` flag trades statistical rigor for speed (fewer
samples), which makes it tempting for fast local iteration. **Don't use it
to decide whether a regression is real.** Testing the same deliberate
regression above with `--quick --baseline main` produced:

```text
sum/1000                time:   [524.63 µs 524.65 µs 524.70 µs]
                        change: [+603844% +604224% +604604%] (p = 0.10 > 0.05)
                        No change in performance detected.
```

Despite an obvious, massive slowdown (86ns → 524µs — the raw numbers make
it visible), the significance test said "no change" because `--quick`'s
reduced sample count pushed `p` just over the 0.05 threshold. Use `--quick`
only while writing/debugging a new benchmark; drop it for the actual
before/after comparison.

## Justfile shortcuts

```bash
just bench-baseline    # save a baseline named "main"
just bench-compare     # compare current code against it
```

## In CI

This is deliberately **not** wired into CI as an automated gate — a
baseline needs to persist across separate CI runs (main's last successful
build vs. a PR's), which means either a cache keyed in a way that's
resistant to poisoning from a bad PR, or a separate storage mechanism.
That's real infrastructure, not a template default; the manual workflow
above is the recommended path until a project's needs justify building it.
