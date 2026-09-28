# Feasibility memo

Research date: 2026-09-27. Recommendation: **proceed with native-client engineering and measurement; treat live Discord user-account compatibility as an unresolved product dependency.**

## What is established

Discord's September 8 patch notes report Electron 42 on desktop, roughly 17% lower p50 CPU use from that upgrade, and roughly 4% higher p50 memory use. Those are Discord's aggregate figures, not measurements of Sean's installation. They support investigating memory overhead, but do not establish that Electron explains every slowdown. [S01](SOURCES.md#s01)

The public OAuth2 scope list has no unrestricted “replace the desktop client” scope. `messages.read` is described for local RPC, while several RPC scopes require partner approval. A bot's permissions and identity differ from a person's account. The self-bot support policy explicitly restricts automating ordinary accounts outside the supported bot/OAuth routes. [S02–S03](SOURCES.md#s02)

Social SDK is a meaningful avenue to investigate: it provides social features including messaging, linked channels, and voice for games. Its communications documentation has approval and integration requirements. These capabilities should not be confused with blanket access to arbitrary servers and DMs in a standalone replacement client. Ask Discord about eligibility and redistribution terms; no request has been sent. [S04](SOURCES.md#s04)

Discord's terms restrict modification and reverse engineering, with an exception where written consent or applicable law permits it. Whether an interoperability exception applies to this project needs jurisdiction-specific review. Account enforcement risk and service fragility remain even if implementation is technically possible. [S05](SOURCES.md#s05)

## Product choices

| Path | What it can prove | Limitation | Recommendation |
| --- | --- | --- | --- |
| Offline native prototype | Rendering, memory retention, interaction design, event processing | No Discord communication | Build now |
| Bot-based test-server integration | Supported REST/Gateway behavior under granted permissions | Acts as a bot; not the user's client | Optional controlled protocol lab |
| Approved Social SDK/partner integration | Documented communication subset with authorized access | Eligibility, scope and redistribution unresolved | Investigate first |
| Unofficial user-account implementation | Potentially broader compatibility | Unsupported endpoints, account/terms risk, ongoing breakage | Separate explicit decision after research |
| Independent messaging service | Freedom over transport and features | Different product; no automatic Discord interoperability | Preserve as a future fork option, not an assumed pivot |

This scaffold deliberately does not implement live authentication. That is a phase boundary, not a claim that all alternative clients are technically impossible or that a bot can stand in for a user.

## Existing work

| Project | Primary-source finding | What to study |
| --- | --- | --- |
| Abaddon | Repository describes C++/GTK3 alternative client with voice | Native UX, platform integration, compatibility burden |
| Vesktop | Repository describes Electron/Vencord desktop client | Linux integration and community expectations; retains browser runtime |
| WebCord | Repository describes Electron-based client without direct Discord API use | Web-client wrapping tradeoffs; outside our native-rendering goal |

See [S14–S16](SOURCES.md#s14). Their existence is not evidence of service authorization or current DAVE compliance. No source was copied, no license compatibility was assumed, and none was installed or benchmarked here.

## Decision gates

1. **Access:** inventory needed capabilities; get a supportable account/session path or document a conscious change in scope. No production user-token handling before this decision.
2. **UI:** demonstrate Windows keyboard navigation, screen-reader semantics, IME, emoji and mixed-script text. Keep egui provisional until this passes.
3. **Performance:** reproduce a matched baseline on the same Windows hardware. Publish raw runs, versions, hardware, settings, and workload differences.
4. **Voice:** prove DAVE interoperability and audio-device recovery before promising a daily-driver release.

## Open questions

- Would Discord approve a standalone native accessibility/performance client, or a narrower companion product?
- Which normal-account events and settings have no supported equivalents?
- How much of observed memory is renderer heap, decoded media, GPU allocations, caches, or native A/V code?
- Can the UI renderer meet complex text and accessibility requirements without compromising idle power?
- Does a maintained Rust DAVE wrapper meet our interoperability and license needs, or should we bind the official C++ library?
- What scope makes a useful daily driver without claiming “near parity” prematurely?
