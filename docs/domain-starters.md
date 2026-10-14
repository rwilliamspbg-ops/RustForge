# Domain Starters

Three small, self-contained example crates showing what the RustForge
test shapes look like applied to real domains — a parser, an async
service, and a CLI. They exist so adopters can see the patterns in
context instead of only in the category crates' fixtures.

| Starter | Domain | Demonstrates |
|---|---|---|
| [`examples/parser-starter/`](../examples/parser-starter/) | arithmetic expression evaluator | unit tests per construct, property tests (panic-freedom + round-trips), a total error enum |
| [`examples/async-service-starter/`](../examples/async-service-starter/) | async in-memory key-value store | `tokio::test` with paused time, concurrent access, background task + graceful shutdown |
| [`examples/cli-starter/`](../examples/cli-starter/) | `wc`-style counting CLI | pure logic in the lib, thin `main`, integration tests spawning the real binary, `--json` output |

## The containment pattern (important)

Every starter is a **standalone workspace of its own**: its
`Cargo.toml` carries an empty `[workspace]` table, which makes it its
own workspace root — the same detachment pattern [`fuzz/`](../fuzz/)
uses. None of them is a member of the RustForge workspace, so:

- The template's default `cargo test --workspace` stays fast and
  dependency-light — the starters (and their `tokio`/`proptest` dev
  dependencies) never enter it.
- `.gitignore`'s `/examples/*/target/` keeps their build artifacts out
  of version control.
- Each copies out wholesale: `cp -R examples/parser-starter
  path/to/your/project` and it builds wherever it lands.

Run a starter with `cargo test` from inside its directory:

```bash
cd examples/parser-starter && cargo test
cd examples/async-service-starter && cargo test
cd examples/cli-starter && cargo test
```

(They can't be built from the repo root as `-p` members — that's the
point of the detachment.)

## Using one as a starting point

The starters are deliberately small (each a few hundred lines) and
MSRV-aligned with the template (1.75, the same dependency pins where
they overlap). To start a real project from one:

1. Copy the starter's directory into your project.
2. Delete the empty `[workspace]` table if your project has its own
   workspace root, or keep it if the copy stands alone.
3. Grow it: the test files show the shapes — extend the unit tests,
   add properties where the input space is large, add trybuild/
   snapshot tests where the domain calls for them (see
   [`docs/adding-tests.md`](adding-tests.md)).

The domain logic is the throwaway part; the test *shape* each starter
demonstrates is the reusable part.
