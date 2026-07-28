# Architecture

## Workspace shape

RustForge is a **virtual workspace**: the root `Cargo.toml` has no
`[package]` of its own, only `[workspace]` + `[workspace.package]` +
`[workspace.metadata.rustforge]`. That's deliberate — this repo is meant
to be copied into (or have its `crates/` folded into) an adopter's own
workspace, which will supply its own root package.

`fuzz/` is a **second, detached workspace** (its own `[workspace]` table
in `fuzz/Cargo.toml`). `cargo-fuzz` needs nightly and its own dependency
resolution, so it's structurally prevented from ever being pulled into
`cargo build --workspace` / `cargo test --workspace`.

```mermaid
graph TB
    subgraph MainWS["Main workspace (Cargo.toml)"]
        core["core-tests<br/><i>shared fixtures/helpers</i>"]
        syntax["syntax-tests"]
        semantic["semantic-tests"]
        edge["edge-cases"]
        perf["performance-tests"]
        fuzzcrate["fuzz-tests"]
        integ["integration-tests<br/><i>tests/e2e.rs</i>"]

        syntax --> core
        semantic --> core
        edge --> core
        integ --> core
        integ --> syntax
        integ --> semantic
        integ --> edge
        integ --> perf
        integ --> fuzzcrate
    end

    subgraph FuzzWS["fuzz/ — detached workspace"]
        fuzztarget["fuzz-tests-fuzz<br/><i>fuzz_targets/utf8_input.rs</i>"]
    end

    fuzztarget -. "path dependency<br/>(nightly only)" .-> fuzzcrate

    style MainWS fill:#1a1a2e,stroke:#4a4a6a,color:#e0e0e0
    style FuzzWS fill:#2e1a1a,stroke:#6a4a4a,color:#e0e0e0
```

`performance-tests` and `fuzz-tests` have no path dependencies of their
own — they're leaves that `integration-tests` pulls together to
demonstrate cross-category testing (see `crates/integration-tests/tests/e2e.rs`).

## Feature flags as the opt-in mechanism

Every category beyond the default (`syntax`, `semantic`, `integration`)
pulls in real ecosystem tooling *only* when its feature is enabled:

```mermaid
graph LR
    async["async<br/>(core-tests, semantic-tests)"] --> tokio[["tokio"]]
    perf["perf<br/>(performance-tests)"] --> criterion[["criterion"]]
    fuzzfeat["fuzz<br/>(fuzz-tests)"] --> proptest[["proptest"]]
    compilefail["compile-fail<br/>(syntax-tests)"] --> trybuild[["trybuild"]]
    nostd["no_std<br/>(core-tests)"] --> corestd[["core-only module,<br/>no extra dep"]]
    edgefeat["edge<br/>(edge-cases)"] --> edgestd[["checked-arithmetic<br/>helpers, no extra dep"]]
```

`[workspace.metadata.rustforge]` in the root `Cargo.toml` mirrors this as
`default_categories` / `optional_categories`, using the same names as the
real `--features` flags — see the comment there.

## CI pipeline

```mermaid
graph TB
    push["push / pull_request"] --> test["test<br/>(stable, beta, nightly)"]
    push --> trybuildjob["trybuild<br/>(stable only)"]
    push --> fuzzbuild["fuzz-build<br/>(nightly only)"]
    push --> msrv["msrv<br/>(pinned to rust-version)"]
    push --> deny["deny<br/>(main workspace + fuzz/)"]

    test -->|"fmt, clippy, test<br/>--features async,no_std,perf,fuzz,edge"| testdetail["excludes compile-fail:<br/>diagnostic text drifts<br/>across toolchains"]
    trybuildjob -->|"cargo test -p syntax-tests<br/>--features compile-fail"| trybuilddetail[" "]
    fuzzbuild -->|"cargo fuzz build"| fuzzdetail[" "]
    msrv -->|"cargo check --workspace,<br/>then test excluding<br/>performance-tests"| msrvdetail["lockfile deleted first:<br/>format v4 needs Cargo >= 1.78"]
    deny -->|"licenses, advisories,<br/>bans, sources"| denydetail[" "]
```

Each job is scoped to one concern on purpose — see "Keep CI jobs narrowly
scoped" in [`docs/best-practices.md`](best-practices.md) for why, and
[`ci/README.md`](../ci/README.md) for the full rationale behind each job.

## Why a virtual workspace, not a template with a root package

Adopters fall into two shapes:

1. **New project**: use this repo as a GitHub template directly. The
   virtual workspace becomes their root `Cargo.toml` as-is; they add their
   own crate(s) as additional `members`.
2. **Existing project**: copy `crates/` (and optionally `fuzz/`) into
   their own workspace, merging `members` lists.

A virtual workspace supports both without modification. This is also why
`examples/` at the repo root holds documentation rather than code — see
[`examples/README.md`](../examples/README.md) — and why root-level
`tests/` stays empty until an adopter's own root package exists to own it.
