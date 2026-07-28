`cargo-fuzz` scaffold for `fuzz-tests`. This is a **detached** workspace
(`fuzz/Cargo.toml` has its own `[workspace]` table) so it never gets pulled
into the main `cargo build --workspace` / `cargo test --workspace` run —
fuzzing needs nightly and its own dependency resolution.

## Targets

- `fuzz_targets/utf8_input.rs` — feeds arbitrary bytes into
  `fuzz_tests::utf8_input`, asserting it never panics regardless of input.

## Running

```bash
cargo install cargo-fuzz   # once per machine
cd fuzz
cargo +nightly fuzz build
cargo +nightly fuzz run utf8_input
```

Corpus and crash artifacts land in `fuzz/corpus/` and `fuzz/artifacts/`,
both gitignored.

## Adding a target

1. Add a `[[bin]]` entry to `fuzz/Cargo.toml`.
2. Add the matching `fuzz_targets/<name>.rs` using the `fuzz_target!` macro
   from `libfuzzer-sys`, exercising a function from `fuzz-tests` (or
   whichever crate you're fuzzing).
3. CI builds every target on nightly via the `fuzz-build` job in
   `.github/workflows/ci.yml`; no separate wiring needed.
