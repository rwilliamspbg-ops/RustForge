# Security Policy

RustForge is a test-suite template: it has no runtime, no network-facing
components, and no production deployment of its own. Security issues here
mostly fall into two categories.

## Reporting a Vulnerability

**Do not open a public issue for a security report.** Instead, use GitHub's
private reporting flow:

1. Go to the repository's **Security** tab.
2. Click **Report a vulnerability** to open a private security advisory.

Include what you'd include in any good bug report: affected file(s)/version,
reproduction steps, and impact. You should get an initial response within a
few business days.

## Scope

- **Template code** (`crates/*`, `fuzz/`): logic errors, unsound `unsafe`
  usage (none is expected — every crate sets `#![forbid(unsafe_code)]`), or
  a fixture/helper that could mislead an adopter into an insecure pattern
  (e.g. logging secrets, weak randomness presented as suitable for auth).
- **Supply chain**: a dependency pinned in `Cargo.lock` with a known
  advisory. Note that `cargo-deny` already runs in CI against the RustAudit
  advisory database (see `deny.toml` and the `deny` job in
  `.github/workflows/ci.yml`) and Dependabot opens PRs for updates
  (`.github/dependabot.yml`) — check those first, since many advisories are
  caught automatically before a report is needed.

## Out of Scope

- Vulnerabilities in third-party dependencies with no RustAudit advisory yet
  — report those upstream instead.
- Issues that only manifest when deliberately disabling this template's
  safety defaults (e.g. removing `#![forbid(unsafe_code)]`) in a fork.
