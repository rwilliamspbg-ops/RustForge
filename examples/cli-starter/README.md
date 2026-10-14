# CLI Starter (example)

A tiny `wc`-style counting CLI — a domain starter showing CLI testing
patterns. See [`docs/domain-starters.md`](../../docs/domain-starters.md)
for the containment pattern these starters share.

```bash
cargo test
echo "hello world" | cargo run -q --          # 1 2 12
printf 'a b\nc\n' | cargo run -q -- --json    # {"lines":2,"words":3,"bytes":6}
```

What it demonstrates:

- **Pure logic in the lib, thin `main`**: `Counts::of`, `parse_args`,
  and `render` are pure functions with unit tests; `main` only wires
  I/O. The real end-to-end behavior is covered by `tests/cli.rs`, which
  spawns the actual binary (`env!("CARGO_BIN_EXE_tally")`) and asserts
  on stdout, stderr, and exit codes — including stdin piping, file
  input, a missing file, and the failure paths.
- **Hand-rolled arg parsing**: no `clap` dependency — for a starter this
  small, hand-rolling keeps the example dependency-free and instantly
  readable, and the parse errors are user-facing strings the tests pin
  exactly.
- **Machine-readable output** (`--json`): the same "structured output
  for automated consumers" idea as
  [`docs/machine-readable-outputs.md`](../../docs/machine-readable-outputs.md),
  in a CLI's smallest form.
