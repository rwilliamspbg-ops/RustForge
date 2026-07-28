# core-tests

Shared fixtures and assertion helpers reused by every other category crate
in the workspace. Start here when adding a helper that more than one
category should use — see "Reusing fixtures" in
[`docs/adding-tests.md`](../../docs/adding-tests.md).

## Contents

- `UserFixture` — a builder-style fixture (`with_username`, `with_password`).
- `default_user_fixture()` — the default fixture; password comes from the
  `RUSTFORGE_TEST_PASSWORD` env var, empty by default (never hardcoded).
- `assert_contains(haystack, needle)` — assertion helper with a clear message.

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
