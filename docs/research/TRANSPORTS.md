# Transport research program

## Constraint

A faster client can reduce local scheduling, parsing, layout and audio-buffer delay. It cannot remove internet propagation, server fanout or service-imposed limits. Changing the language does not change the protocol spoken by Discord's servers.

| Workstream | Compatible opportunity | How to test |
| --- | --- | --- |
| HTTPS REST | Reuse pools/TLS sessions, cap concurrency, cancel stale fetches; negotiate only server-supported HTTP versions | Timing decomposition on controlled fixtures, then permitted service requests |
| Gateway WebSocket | Incremental parsing, bounded buffers, compression tradeoff, prompt heartbeat and resume | Recorded synthetic bursts, reconnect/fault injection, parse CPU/bytes |
| Voice UDP | Buffer scheduling, loss concealment, jitter control, efficient codec/encryption boundaries | Controlled two-peer audio lab, loopback and impaired links |
| HTTP/3 / QUIC | Potential controlled-endpoint experiment | Must have a compatible server; measure handshake, head-of-line effects and CPU |
| WebTransport/custom relay | Optional independent-service research | Explicitly outside Discord compatibility; relay adds a hop and trust boundary |

Discord's documented voice flow uses a separate WebSocket and UDP media path; voice Gateway v8 is recommended. DAVE is required for eligible A/V sessions from March 1, 2026. Transport encryption and DAVE frame encryption are distinct layers. Plan interoperability using the official whitepaper and libdave before selecting a Rust wrapper. Do not implement custom cryptography or assume a pre-DAVE voice library is sufficient. [S08–S09]

## Lab design

Build a separate two-peer harness using synthetic payloads and opt-in test audio. Compare baseline WebSocket/TCP, raw UDP with explicit loss accounting, and QUIC streams/datagrams where appropriate. QUIC is specified in RFC 9000; support must exist at both endpoints. [S20]

Sweep RTT 0/20/80/150 ms, jitter 0/5/20 ms, random loss 0/1/3/5%, packet reordering, bandwidth caps and reconnect events. Separate establishment latency, warm steady-state one-way delay, p50/p95/p99, bytes, CPU, allocation rate and power. One-way network timing needs synchronized clocks or same-host simulation; RTT/2 is only an estimate. Reliable-message and real-time-media semantics are different workloads.

For voice, measure capture-to-playback latency with loopback equipment and quantify audio quality at each buffer size. Optimize callback allocations, device switching and jitter behavior before chasing protocol novelty. A shorter buffer may reduce latency while worsening dropouts; record both.

This repository contains no QUIC implementation, voice stack, relay or performance result for any transport. That work is explicitly scheduled after the access and native-UI gates.
