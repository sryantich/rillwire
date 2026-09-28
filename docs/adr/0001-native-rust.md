# ADR 0001 — Rust core, provisional egui desktop

Status: accepted for M0; renderer choice revisited at accessibility/text gate.

| Candidate | Strength | Tradeoff | Decision |
| --- | --- | --- | --- |
| Rust + egui/eframe + wgpu | Direct GPU UI, explicit lifetimes, native Windows target, accessible integration via AccessKit | Complex text, rich chat, variable-height virtualization and screen-reader behavior need real validation | M0 implementation |
| Rust + Iced | Message-driven application model, portable native rendering | Rich text, virtualization and accessibility still require a targeted spike | Primary alternative |
| Go + Gio | Productive concurrency and native rendering across platforms | GC pacing/allocation costs and media FFI need measurement, not assumptions | Viable alternative |
| Go + Fyne | Native toolkit and broad OS tooling | Custom chat rendering and advanced media integration require investigation | Secondary candidate |
| Rust + Tauri / Go + Wails | Mature web layout and easier web reuse without bundled Chromium | Still uses an OS WebView; changes rather than eliminates web-runtime overhead | Outside initial native-UI goal |
| Rust + Windows-only UI | Direct OS integration | Cross-platform UI investment duplicated later | Platform-specific services only |

Choose Rust because predictable ownership, low-level media integration, and control over hot-path allocation match this project's priorities. This is an engineering preference, not a measured claim that Go is too slow. Go's collector has CPU/memory/latency tradeoffs that can be tuned [S13]; Rust can also allocate excessively or block its UI thread.

egui provides a quick instrumentable native prototype [S10]. We enable AccessKit and wgpu and request redraws for actual state changes, not a permanent timer. Immediate-mode UI does not require a continuous 60 Hz idle loop. Renderer/GPU memory remains part of the budget. Iced [S11] and Gio [S12] remain reference alternatives. Tauri uses WebView2 on Windows [S17], and Wails uses the OS WebView [S18].

Before choosing egui for M1, test bidi/RTL, CJK shaping, emoji, IME composition, selection/copy, links, NVDA navigation, 100–300% DPI and variable-height history. If those fail materially, retain the Rust core and replace the UI. Do not remove accessibility to meet a memory goal.

M0 pins eframe 0.36.2 and Rust 1.98.1. `Cargo.lock` controls transitive dependencies. Update intentionally with a compile/interaction/performance comparison. No async runtime or database dependency is added merely to make the dependency diagram look ambitious.

Source keys refer to [the research register](../research/SOURCES.md).
