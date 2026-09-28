# Contributing

Start with the PRD and roadmap. Keep changes focused and state the user behavior, performance hypothesis, verification performed and known limits.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run -p rillwire-lab --release --locked -- replay
```

Use synthetic fixtures; do not commit account tokens, message captures, cookies, ETL/HAR/packet dumps or machine-specific credentials. Preserve the existing API-access milestone rather than representing unfinished connectivity as working. Dependency licensing and security review precede release.

A performance PR needs a reproducible workload and before/after measurements. Report hardware, build, power/display settings and all trials. Avoid turning a container microbenchmark into a Windows client marketing claim.
