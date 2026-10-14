# Parser Starter (example)

A tiny arithmetic-expression evaluator — a domain starter showing the
template's test shapes applied to a real parser. See
[`docs/domain-starters.md`](../../docs/domain-starters.md) for the
containment pattern these starters share.

```bash
cargo test        # unit + property tests
```

Quick eval from the doc example:

```rust
use parser_starter::eval;
assert_eq!(eval("1 + 2 * 3"), Ok(7));
```

What it demonstrates:

- **Unit tests per construct** — precedence, associativity, parens,
  unary minus, integer division, whitespace, every error variant.
- **Property tests** (`proptest`) — arbitrary strings never panic
  (the `Result`-based API's whole point), and generated operand pairs
  round-trip through format → parse → eval.
- **A total error enum** — no panics anywhere: division by zero and
  every overflow path return `EvalError` variants, which the property
  tests verify holds for *all* input.
