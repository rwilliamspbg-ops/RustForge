## What and why

<!-- What does this change, and why does it belong in the template
     (see CONTRIBUTING.md)? -->

## Checklist

- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --workspace --all-features` passes
- [ ] `cargo test -p syntax-tests --features compile-fail` passes on stable, if `syntax-tests`' UI fixtures changed
- [ ] New optional tooling is behind a Cargo feature and MSRV-pinned if needed (see README's "MSRV" section)
- [ ] `README.md` / `docs/adoption.md` updated if this adds or changes a category, feature flag, or CI job
- [ ] `CHANGELOG.md` updated under `[Unreleased]`

<!-- Or just run scripts/check.sh, which covers the first three items. -->
