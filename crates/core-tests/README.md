# core-tests

Shared fixtures and assertion helpers reused by every other category crate
in the workspace. Start here when adding a helper that more than one
category should use — see "Reusing fixtures" in
[`docs/adding-tests.md`](../../docs/adding-tests.md).

## Contents

- `UserFixture` — a builder-style fixture (`with_username`, `with_password`).
- `default_user_fixture()` — the default fixture; password comes from the
  `RUSTFORGE_TEST_PASSWORD` env var, empty by default (never hardcoded).
- `ConfigFixture` — the same builder pattern for config-shaped test data
  (`base_url`, `timeout_ms`, `retries`).
- `assert_contains(haystack, needle)` — assertion helper with a clear message.
- `retry(max_tries, attempt)` — retries a fallible closure, for testing
  flaky/eventually-consistent operations without an async runtime.
- All of the above have runnable doc examples (`cargo test --doc -p core-tests`).
- `tests::retry_table_driven_examples` — a table-driven test pattern worth
  copying for similar "many small variations of one behavior" cases.

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `async` | `tokio` | `async_support` — async-friendly fixture helpers |
| `no_std` | — | `no_std_support` — `core`-only helpers |

## Example

```bash
cargo run -p core-tests --example fixture_walkthrough
```

See the root [`README.md`](../../README.md) for the full workspace overview.
