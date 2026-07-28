# Fuzzing and Corpus Management

Advanced usage beyond "add a target" (covered in [`fuzz/README.md`](../fuzz/README.md))
and "when to reach for `proptest` vs `cargo-fuzz`" (covered in
[`docs/adding-tests.md`](adding-tests.md)).

## What CI actually runs

The `fuzz-build` job (see [`ci/README.md`](../ci/README.md)) builds every
target and does a **10-second smoke run** per target on every push/PR,
seeded from the committed `fuzz/seed_corpus/<target>/`. That's enough to
catch a broken harness (panics immediately on its own seeds) — it is
**not** a real fuzzing campaign. Real bug-finding needs minutes to hours,
not seconds; that's what the local workflow below and the nightly extended
job are for.

## Running a real campaign locally

```bash
cd fuzz
cargo +nightly fuzz run utf8_input               # runs until you Ctrl-C
cargo +nightly fuzz run utf8_input -- -max_total_time=300   # 5 minutes
cargo +nightly fuzz run utf8_input -- -jobs=4 -workers=4    # parallel
```

New inputs libFuzzer finds get written to `fuzz/corpus/<target>/`
(gitignored — it grows unboundedly and isn't meant to be committed; the
curated `fuzz/seed_corpus/<target>/` is the committed starting point, kept
deliberately small).

## If it finds a crash

libFuzzer writes the crashing input to `fuzz/artifacts/<target>/` and
prints a path. Reproduce it directly:

```bash
cargo +nightly fuzz run utf8_input fuzz/artifacts/utf8_input/crash-<hash>
```

Minimize it to the smallest input that still reproduces the crash before
turning it into a regression test — a 40KB crash input is much harder to
reason about than the 3 bytes it minimizes to:

```bash
cargo +nightly fuzz tmin utf8_input fuzz/artifacts/utf8_input/crash-<hash>
```

Once minimized, that input is exactly what you want as a new
`fuzz/seed_corpus/<target>/` entry (so the bug stays covered by the CI
smoke-run going forward) and/or a new `#[test]` case in the corresponding
crate's `src/lib.rs` unit tests.

## Corpus minimization

Over a long campaign, `fuzz/corpus/<target>/` accumulates inputs that are
redundant — they don't add new code coverage over what's already there.
Minimize it periodically (this only touches the local, gitignored
`corpus/` dir, never `seed_corpus/`):

```bash
cargo +nightly fuzz cmin utf8_input
```

## Coverage-guided sanity check

To see what the corpus actually covers (useful for spotting a target
that's stuck exploring one narrow path):

```bash
cargo +nightly fuzz coverage utf8_input
```

This needs `llvm-tools-preview` (`rustup component add llvm-tools-preview`)
and produces a coverage report under `fuzz/coverage/<target>/`.

## Reading Address Sanitizer output

`cargo fuzz` builds with ASan by default. A typical report has the crash
type up top (`heap-buffer-overflow`, `use-after-free`, ...) followed by
the allocation/deallocation stack traces — read those two stacks (where it
was allocated/freed vs. where the bad access happened) before the crash
site itself; that's usually where the actual bug is, not the line ASan
points at first.

## Extended nightly campaign

Beyond the 10-second push/PR smoke-run, the `fuzz-build` job also runs a
longer campaign (currently a few minutes per target) on the daily
scheduled (`cron`) trigger only — real bugs that need more than 10 seconds
of fuzzing to surface get a chance to, without slowing down every PR. See
the `if: github.event_name == 'schedule'` step in
`.github/workflows/ci.yml`.
