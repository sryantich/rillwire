# API and capability research map

Last reviewed 2026-09-27. Documentation-level inventory; no authenticated service requests were made.

| Surface | Publicly documented purpose | Research/implementation work | M0 |
| --- | --- | --- | --- |
| HTTP API v10 | Resource reads and writes under an allowed identity | Versioning, permissions, paging, headers, CDN limits | No HTTP client |
| OAuth2 | Granted scopes for app/user integrations | Exact capability map, partner access, system-browser flow | No login |
| Gateway v10 | Stateful secure WebSocket events | READY/resume, ACK timeout, sequence, intents, close codes, sessions | Subset parser + heartbeat timer only |
| REST rate limits | Server-defined buckets/global pauses | Composite major-resource key, proactive admission, 429 recovery | Retry-After gate only |
| Voice Gateway v8 | Separate signaling channel | State transitions, sequence acknowledgements, device recovery | Research only |
| Voice media | UDP media transport and encryption | Negotiation, codec/RTP timing, packet loss, DAVE | Absent |
| Social SDK | Game social graph and communication features | Standalone eligibility, scopes, redistribution, feature gaps | Research only |
| CDN/media | Assets and attachments | Permission changes, expiring URLs, content size/type and pixel budgets | Absent |
| Local RPC | Local client integration with approved scopes | Whether it requires official client to remain running | Research only |
| Private client API surfaces | Unknown/unsupported for this project | Record missing capabilities without assuming bot equivalents | No enumeration or implementation |

Gateway v10 uses secure WebSockets; public documentation also describes JSON/ETF and zlib/zstd stream options. These are server-negotiated choices, not evidence of arbitrary transport support. The parser's 1 MiB inbound cap is our local budget, not Discord's outgoing-message limit. [S06]

The retry gate rounds fractional server delays up, combines per-bucket and global deadlines, and never shortens an existing pause. It is **not** a complete rate limiter: remaining/reset counters, route-to-bucket discovery, credential separation, session-start limits, and request scheduling are not implemented. [S07]

For each future endpoint, record method/path template, identity/scope, permission, major resource, pagination, cache invalidation, rate-limit evidence, redacted fixture and source revision. Avoid a single “supports Discord” boolean: expose capabilities such as `read_channel`, `send_message`, `read_dm`, `join_voice`, and `search_remote` separately.

Sources: [register](SOURCES.md).
