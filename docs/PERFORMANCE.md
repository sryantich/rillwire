# Performance contract

All numbers below are **initial engineering targets, not achieved results**. Revisit after the baseline. Reference machine: Windows 11 x64, 4+ modern CPU cores, 16 GiB RAM, integrated or discrete DirectX 12 GPU, NVMe, 1920×1080 at 60 Hz, 100% scaling, balanced power. Also run on Sean's desktop as a separate hardware profile.

| Metric | Initial target | Measurement |
| --- | --- | --- |
| Text idle private bytes, all app processes | ≤150 MiB after settling | Process private bytes; report GPU allocations separately |
| Text workload peak private bytes | ≤250 MiB | Fixed 20-channel fixture, bounded thumbnails |
| Idle visible CPU | ≤1% of one logical CPU on average | CPU-time delta / wall time; also report machine-normalized % |
| Minimized idle | Event driven, no app-owned animation loop | Wakeup/CPU ETW capture; compare visible state |
| Warm launch to interactive, p95 | ≤1 second | External launch marker → first usable frame, 30 trials |
| Cold launch to interactive, p95 | ≤2 seconds | Separate fresh-session/cache-state protocol, 30 trials |
| Cached navigation input-to-paint, p95 | ≤16.7 ms | Input timestamp → presented frame; 100+ actions |
| Long-soak retained memory | No sustained growth after workload plateaus | 2-hour cycles, return to idle, fitted private-byte slope |
| Cache retention | ≤4,096 entries and ≤4 MiB text in M0 | Invariant tests; allocator/index/GPU overhead excluded |

FPS alone does not establish responsiveness. Report frame pacing and p99 stalls too. Native rendering removes the browser runtime but does not eliminate GPU, font atlas, media, filesystem or allocator overhead. Do not add RSS/private bytes/working set together: these are different views of memory. Summing working sets across processes double-counts shared pages.

## Baseline method

Record build hashes, release/debug, OS/driver versions, power mode, DPI, window size, history/media fixtures, network and feature settings. Measure all related processes and GPU memory. Alternate run order to reduce thermal/cache bias. Distinguish cold, warm, settled and steady-state. Publish individual trials and median/p95/p99, not only the best run.

`Measure-Client.ps1` samples all processes with one executable name. Confirm whether a tested client starts differently named helpers and measure those separately; it does not claim full process-tree attribution. `CPU_one_core_percent` can exceed 100% for multithreaded work. `CPU_machine_percent` divides that by logical processor count.

The included lab benchmark measures cache insertion with synthetic message creation on the host that runs it. It excludes parsing, rendering, network, RSS and audio. The fixture replay is a correctness smoke check. Neither is a comparison to Discord or evidence that the Windows targets are met.

## Next measurements

1. Capture stock Discord with the published scenarios and preserve raw local results.
2. Capture the native M0 shell for matching idle/navigation work; label missing live features.
3. Profile the largest differences before adding dependencies or optimizing code.
4. Establish budgets for text layout, decoded images, avatars, GPU textures, history, socket queues and audio.
5. Re-run matched workloads at every milestone. Track performance regressions separately from functional CI.
