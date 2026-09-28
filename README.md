# Rillwire

**Conversation, in its element.**

An independent, open-source native-client research project for a lighter Discord experience. Windows 11 x64 first; portable Rust core; no Electron or WebView in the desktop app.

**v0.1 is an offline research scaffold, not a working Discord replacement.** The native demo supports channel browsing, cached-message search, local message composition, virtualized compact rows, and bounded stress loading. It does not sign in, access Discord, send remote messages, or make calls. There are no performance claims against Discord yet.

## Start here

- [Research and feasibility](docs/research/FEASIBILITY.md)
- [Product scope and feature matrix](docs/PRD.md)
- [Architecture](docs/ARCHITECTURE.md) and [Rust/UI decision](docs/adr/0001-native-rust.md)
- [Windows profiling plan](docs/research/CLIENT-TRACING.md)
- [Transport research](docs/research/TRANSPORTS.md) and [performance targets](docs/PERFORMANCE.md)
- [Prioritized roadmap](docs/ROADMAP.md), [verification record](docs/VERIFICATION.md), and [integration handoff](docs/INTEGRATIONS.md)

## Run on Windows 11 x64

Install Rust with the MSVC toolchain using [rustup](https://rustup.rs/), plus Visual Studio Build Tools with **Desktop development with C++** and a Windows SDK. This workspace pins Rust 1.98.1. From the repo root:

```powershell
cargo run -p rillwire-desktop --release
cargo test --workspace
cargo run -p rillwire-lab --release -- replay
cargo run -p rillwire-lab --release -- bench 100000
```

The desktop uses egui/eframe 0.36.2 and wgpu. `Ctrl+K` focuses search. The load button feeds 50,000 synthetic messages on a worker through a 128-message queue; UI work is limited to 256 messages per update. Message retention is capped globally at 4,096 entries and 4 MiB of UTF-8 payload. These limits are **not** an RSS guarantee. Closing the demo clears messages.

Linux development requires a C compiler, pkg-config, and the X11/Wayland development libraries. macOS and Linux are architectural targets; only tested platforms listed in the verification record should be treated as verified.

## Workspace

| Path | Current responsibility |
| --- | --- |
| `apps/desktop` | Native offline UI and synthetic load worker |
| `apps/lab` | Fixture replay and cache insertion microbenchmark |
| `crates/rillwire-core` | Domain boundary, bounded cache, heartbeat timing, 429 retry gate |
| `crates/rillwire-protocol` | Size-bounded parser for a small documented Gateway subset |
| `fixtures` | Synthetic data only |
| `tools` | Windows measurements, result summary, backlog export |
| `site` | Standalone branding/research site prepared for Vercel; separate from desktop |

## Compatibility comes before login

Public bot APIs and OAuth2 are not a documented general-purpose replacement-client authorization path. Social SDK provides communications for approved game integrations; its applicability to a standalone client needs confirmation. Discord also restricts self-bots and reverse engineering in its published terms, with stated exceptions. See the sourced feasibility analysis before implementing live user access.

A future Discord adapter must preserve service capabilities, permissions, rate limits, and encryption. QUIC experiments belong to independently controlled lab endpoints; selecting QUIC locally cannot change Discord's server protocol. No Discord code, assets, or credentials are included.

## Contribute

Read [CONTRIBUTING.md](CONTRIBUTING.md). This repository is MIT licensed. Rillwire is a working name, not a cleared trademark, and is not affiliated with or endorsed by Discord. Third-party dependencies retain their own licenses.
