# Roadmap

Estimates are planning ranges for one experienced developer with assistance, not delivery commitments. Access approval may dominate calendar time. The machine-readable backlog is `tools/backlog.json`; the CSV is a handoff artifact, not evidence of a Linear import.

| Stage | Planning range | Exit evidence |
| --- | --- | --- |
| M0 research foundation | Initial scaffold delivered; 1–2 additional weeks of real-machine study | Source register, baseline captures, native-UI gate, access decision |
| M1 text client | Approximately 6–10 engineering weeks after a viable access path | Authorized end-to-end text workflow, storage, reconnect, accessibility, measured budgets |
| M2 voice beta | Additional 4–8+ weeks | DAVE interoperability, reliable audio/device switching, loss recovery, Windows soak |
| M3 richer client | Scope separately | Screen/video capture, threads, rich media, platform expansion |

## Ordered backlog

| Key | Priority | Work | Depends on | Acceptance |
| --- | --- | --- | --- | --- |
| RW-001 | Urgent | Resolve account/API access and SDK eligibility | — | Written capability decision; permission/terms risks and product scope recorded |
| RW-002 | High | Capture current Windows Discord baseline | — | Versioned scenario manifest, 30 launch trials, idle/navigation/soak data |
| RW-003 | High | Validate native UI accessibility and text | — | NVDA, IME, RTL, emoji, DPI report; egui/Iced decision |
| RW-004 | High | Complete gateway replay/session state machine | — | READY/RESUMED, close codes, sequence, jitter, faults and compression fixtures |
| RW-005 | High | Implement bounded REST scheduler | RW-001 | Route/major/global buckets, cancellation, retry policy, no duplicate writes |
| RW-006 | High | Implement selected auth and credential storage | RW-001 | System-browser flow, logout/revoke, OS secret storage, no secret logs |
| RW-007 | High | Build variable-height timeline and delivery states | RW-003, RW-006 | Paging, replies/edits/deletes, selection, pending/failed/sent semantics |
| RW-008 | Normal | Add durable bounded cache | RW-006 | SQLite migration, disk quotas, permission/deletion purge, corruption recovery |
| RW-009 | Normal | Add native notifications/tray | RW-006 | Mute/DND respected, private toast settings, no lost exit state |
| RW-010 | High | Prove DAVE interoperability | RW-001 | Library/license selection, vectors and eligible-session integration proof |
| RW-011 | High | Build Windows voice/audio lab | RW-010 | WASAPI/Opus, callback budgets, PTT/mute/deafen, unplug/replug/loss recovery |
| RW-012 | Normal | Compare controlled transports | RW-002 | Two-peer harness, reproducible impairments, CPU/latency/quality results |
| RW-013 | High | Package and release securely | RW-003, RW-006 | Signed installer/update threat model, checksums, dependencies/license review |
| RW-014 | Normal | Publish research site | — | Reviewed static content, approved Vercel destination, mobile/desktop check |
| RW-015 | High | Enforce performance budgets | RW-002, RW-007 | Repeatable Windows matched-workload report with raw runs and regression gate |
| RW-016 | Low | Plan other OS/architectures | RW-015 | Per-target build/runtime gaps; no untested support claim |

## First next work session

Run M0 on Windows, take the idle/navigation baseline for both apps, and test IME/NVDA before polishing the renderer. In parallel in the project schedule, resolve API eligibility. No actual delegated agents or external outreach are part of this scaffold.
