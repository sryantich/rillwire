# Integration handoff

Status as of 2026-09-27. No external messages have been sent.

## GitHub

- Authenticated owner verified: `sryantich`.
- Proposed new public repository: `sryantich/rillwire` (working project name).
- Prepared: local `main` branch, source, lockfile, MIT license, documentation, issue/PR templates and Windows/macOS/Linux CI.
- Pending: repository creation and push. The connected GitHub toolset supports existing repositories but exposes no create-repository action. Browser fallback requires permission under the browser tool's integration rules. No existing repository was reused or modified.
- The delivery includes a git bundle so the prepared commit survives independently of the scratch workspace.

After creating an empty repository, from a local checkout:

```sh
git remote add origin https://github.com/sryantich/rillwire.git
git push -u origin main
```

Or with an authenticated GitHub CLI, after reviewing the source and public visibility:

```sh
gh repo create sryantich/rillwire --public --source . --remote origin --push --description "Native Rust client research for a lighter Discord experience. Windows first; offline prototype."
```

Do not add a second origin if one already exists. CI is prepared but has not run on GitHub; the Windows artifact is not yet available. Enable private vulnerability reporting after creation, and choose branch-protection rules once the first successful CI run exists.

## Linear

The connected tool returned `UNAUTHORIZED` and explicitly requested reauthentication. No workspace/team/project/issue was created or changed.

Prepared project: **Rillwire**. `tools/backlog.json` contains the project description plus 16 issues with priority, acceptance criteria and local dependency keys. `tools/linear-backlog.csv` is a readable import/handoff form. The `RW-xxx` keys are local planning references, not actual Linear issue IDs.

After reconnecting: list available teams, choose the appropriate product team, create or match the project, create issues once, map local keys to returned Linear IDs, then apply dependencies. Do not retry blind after ambiguous mutation results. CSV priority/status mapping should be reviewed against the actual workspace import UI.

## Vercel

Verified team: **Beard Byte Labs** (`beard-byte-labs`). Proposed project: **rillwire**.

Only `site/` is intended for deployment. Framework: Other/static. Root directory: `site`. No build/install command, no environment secrets, no backend, no analytics or Discord connectivity. Keep deployment protection enabled where available. Review the visual concept and project status text before publication.

An initial generic deploy-tool action was rejected by automatic approval review because the content/destination were unspecified and publication had not been authorized. No deployment was created. The complete, reviewable static site is now supplied for a specific publication decision; the rejected action has not been retried or bypassed.

After explicit approval, link the directory to the intended team/project, deploy a preview, verify desktop/mobile interactions and headers, then decide whether to promote it. Do not deploy the entire Rust repository or local research captures. Once GitHub exists, update the site's project-status copy with the real repository link.
