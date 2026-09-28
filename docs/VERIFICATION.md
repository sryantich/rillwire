# Verification record

Date: 2026-09-27. Host: Ubuntu 24.04 x86_64 container. Toolchain: Rust 1.98.1. Desktop framework pinned to eframe 0.36.2. No Discord account, backend connection or Windows desktop was accessed.

| Check | Outcome | What this establishes |
| --- | --- | --- |
| `cargo fmt --all --check` | Pass | Workspace formatting |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Pass | Host lint/type checks without warnings |
| `cargo test --workspace --locked` | Pass: 13 tests | Cache, timing/retry, parser and headless native UI behavior |
| `cargo check -p rillwire-desktop --locked` | Pass | Linux-target desktop compilation check |
| `cargo check -p rillwire-desktop --locked --target x86_64-pc-windows-msvc` | Pass | Windows x64 target type/compilation check; no Windows linking or execution |
| `cargo run -p rillwire-lab --release --locked -- replay` | Pass | 8 synthetic frames; 1 retained message; latest dispatch sequence 4 |
| `cargo run -p rillwire-lab --release --locked -- bench 100000` | Completed | 5 measured cache-insert runs, 1 warmup; 4,096 messages retained |
| `node --check site/app.js` | Pass | JavaScript syntax only |
| Python metrics summary | Syntax and synthetic-case check pass | CSV/JSON processing; not Windows collection validation |
| Manifest/asset checks | Pass | TOML/JSON/SVG parse and local Markdown links resolve |
| Browser visual check | Not completed | Browser installation failed: certificate trust issue in agent-browser and unusable Playwright browser download; no screenshot or browser interaction claim |

## Tests

- Cache: cross-channel global eviction, duplicate update handling, oversized-edit rejection without losing the old value, UTF-8 byte accounting, deletion and zero capacity.
- Heartbeat: initial delay, periodic ACK, requested heartbeat, missing ACK, failed-state persistence and invalid intervals.
- Retry gate: fractional delays rounded up, major-resource separation, global pause, no shortened deadline and malformed delay rejection.
- Parser: synthetic replay, large snowflake fidelity, unknown dispatch sequence, unknown opcode, malformed and oversized frames.
- Native UI: local composition, channel-scoped search and headless egui frame generation. Texture uploads are explicitly discarded because this test has no GPU surface.

## Microbenchmark evidence

The recorded median was 23.685 ms for 100,000 synthetic cache insertions in a release build on this shared Linux container. Retained text payload was 286,720 bytes across 4,096 entries. Raw samples are in `docs/evidence/cache-benchmark-linux.json`.

This measurement includes string creation and cache insertion, and excludes parser/UI/GPU/network work and process memory. It is not a Windows result, a Discord comparison, or a stable performance guarantee. Concurrent build activity and shared-host scheduling may influence the sample spread.

## Unverified

Windows executable linking and runtime, GPU presentation, NVDA/IME/RTL/high DPI, real input-to-paint timing, idle CPU/power, process memory, Windows PowerShell collection, installer/signing, live APIs, authentication, actual rate-limit behavior, reconnect/resume completeness, voice/video and DAVE. macOS builds have not run locally. GitHub Actions are configured but not executed remotely.

The web page is an interactive design concept separate from the native app. Its JavaScript has syntax validation, but browser layout and interactions remain unverified in this environment. Current dependency/download limitations do not alter that status.
