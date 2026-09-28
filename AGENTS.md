# Repository guidance

- Read README.md and docs/VERIFICATION.md before modifying the scaffold.
- Preserve the distinction between implemented offline behavior and planned live adapters.
- Keep core independent of UI and network dependencies; bound queues and retained data.
- Use synthetic data. Never commit raw client captures or credentials.
- Update docs/VERIFICATION.md with actual commands and limitations, not assumed passes.
- Verify relevant behavior and formatting before committing; do not expand product scope silently.
- Do not claim Windows runtime performance from Linux builds or microbenchmarks.
