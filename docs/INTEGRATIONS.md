# Integration status

Updated 2026-09-29.

## GitHub

- Public repository: [sryantich/rillwire](https://github.com/sryantich/rillwire).
- Default branch: `main`. Initial scaffold commit: `77d122f5678ec7df3a98054a0c3bcc9d08fd0445`.
- [Rust CI run #1](https://github.com/sryantich/rillwire/actions/runs/36376314848) passed all four jobs: core tests and fixture replay on Windows, macOS and Linux, plus Windows desktop formatting, linting, workspace tests and a release build for `x86_64-pc-windows-msvc`.
- That run provides `rillwire-windows-x64-unsigned-preview` (14-day retention, expires 2026-10-12). It contains the executable, README, license and executable SHA-256. This is an offline research preview, not a signed installer or a live Discord client.
- Windows interactive behavior and performance remain unverified. See [VERIFICATION.md](VERIFICATION.md).

For a new checkout:

```sh
git clone https://github.com/sryantich/rillwire.git
cd rillwire
```

Branch protection and private vulnerability reporting have not been configured.

## Linear

Deferred at the project owner's request because the connection requires reauthentication. No Linear project or issues have been created.

`tools/backlog.json` contains the project description and 16 issues with priority, acceptance criteria and local dependency keys. `tools/linear-backlog.csv` is an import/handoff form. `RW-xxx` keys are local planning references, not Linear issue IDs.

When work resumes: resolve the intended team, create or match the project, create issues once, map local keys to returned IDs and apply dependencies. Review CSV mappings against the actual workspace before import.

## Vercel

Destination: **Beard Byte Labs** (`beard-byte-labs`). Planned project: **rillwire**. Publication has been authorized, but no project or deployment has been created yet.

Deploy only `site/`, the concept and branding page with a synthetic interactive chat mockup. The Rust desktop client is a separate build. There is no Discord login, live messaging, backend, analytics, signup or secret configuration in the site.

Import the existing GitHub repository with these settings:

| Setting | Value |
| --- | --- |
| Repository | `sryantich/rillwire` |
| Production branch | `main` |
| Framework | Other/static |
| Root directory | `site` |
| Build and install commands | None |
| Output directory | `.` |
| Environment variables | None |

The connected deployment action was unavailable. Browser fallback is authorized. On 2026-09-29 the browser initially displayed the signed-in team dashboard, but project navigation and a fresh dashboard request returned to login. Authentication remains the current blocker.

After importing, verify the deployed page, its local demo interactions and the response headers from `site/vercel.json`, then record the actual project and deployment URLs here. Keep the default preview protection. Do not deploy the Rust repository root or research captures. The site already links to the real GitHub repository.
