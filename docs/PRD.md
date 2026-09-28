# Product brief and feature matrix

## Intent

Build a responsive, understandable native conversation client for Windows 11 x64, with bounded resource consumption and a reusable cross-platform core. Users should browse communities, read and write messages, and eventually join voice with less overhead. Connectivity to Discord is conditional on the access decision in the feasibility memo.

**MVP is a text-first daily workflow, not “90% of Discord.”** Feature count is a poor proxy: voice, screen sharing, accessibility, moderation, permissions, and authentication each carry disproportionate effort. The deliverable in this repository is M0, an earlier offline engineering preview.

## Releases

| Capability | M0 scaffold now | M1 text MVP | M2 voice beta | Later |
| --- | --- | --- | --- | --- |
| Windows native shell | Implemented | Polish and accessibility gate | Maintain | ARM64/macOS/Linux |
| Channel navigation | Synthetic channels | Authorized guild/channel list | Maintain | Folders and customization |
| Message timeline | Compact virtualized previews | Variable-height text, paging, edits, deletes, replies, unread markers | Maintain | Rich embeds and threads |
| Compose/send | Local echo only | Delivery states, deduplication, retry UX, upload limits from service | Maintain | Rich editing |
| Search | Current retained channel only | Cached local search with coverage indicator | Maintain | Supported server search, if available |
| Authentication | Absent | Approved/supportable adapter decision required | Same | Multiple accounts if supported |
| DMs/friends/presence | Absent | Only where selected access path supports them | Expand with evidence | Full parity not promised |
| Attachments/emoji | Absent | Download on demand, capped decode/cache, safe links, basic emoji | Expand | GIFs/stickers, previews |
| Notifications/tray | Absent | Opt-in native toasts, mute/DND, notification privacy | Maintain | Global shortcuts |
| Voice/audio devices | Disabled UI placeholder | Architecture only | Capture/playback, Opus, DAVE, mute/deafen/PTT, reconnect | Noise processing refinement |
| Video/screen sharing | Absent | Out of scope | Out of scope | Separate capture/codec/DAVE program |
| Moderation/admin/settings | Absent | Read permissions accurately; supported small subset | Incremental | Advanced moderation |
| Nitro, shop, payments, quests, games, overlays | Absent | Out of scope | Out of scope | No commitment |
| Updates/installer | Source only | Unsigned CI artifact for internal testing | Signed installer and authenticated updates before public auto-update | Platform packaging |

## M1 acceptance

- A user can complete the agreed text workflow on the selected access path; unsupported capabilities are explicit.
- Reconnection does not silently duplicate outgoing messages or lose sequence state; delivery status reflects remote acknowledgement.
- Caches have global entry/byte/disk limits, and permissions/deletions invalidate retained content.
- UI remains usable while media decodes, network retries, and history loads run in bounded workers.
- Keyboard-only navigation, NVDA, IME, high DPI, multiple monitors, reduced motion, and contrast are verified.
- Windows performance targets are measured under a published reproducible protocol. Failures are visible and analyzed.
- Secrets use OS facilities; logs do not contain tokens or message content by default.

## Nonfunctional priorities

Correctness and accessibility come before a marketing memory number. Favor predictable allocations, bounded queues, on-demand media, event-driven redraws, and fast cached navigation. A language rewrite by itself guarantees none of these. Telemetry is off by default; diagnostic export is explicit and scrubbed.

## Scope control

M0 can proceed without Discord access. M1 cannot be represented as a Discord client release until authentication and capability coverage are resolved. If the supported route only permits a companion app, rename the milestone and reapprove its scope instead of quietly replacing the original goal.
