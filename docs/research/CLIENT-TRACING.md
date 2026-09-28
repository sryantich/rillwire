# Windows client observation and tracing plan

No access to Sean's Windows desktop was available in this session. No Discord process, memory dump, authenticated traffic or backend endpoint was traced. The scripts here prepare a reproducible local study.

## Baseline setup

Use a dedicated Windows 11 test profile and a server with consenting test participants and synthetic messages. Record OS build, Discord build/channel, Electron version where exposed, CPU/GPU/drivers, RAM, power plan, display refresh/DPI, peripherals and network. Freeze a workload manifest. Compare the same feature workload on the same hardware; an offline text prototype and a full live A/V client are not equivalent workloads.

Start with aggregate process measurements and UI observations. Use Microsoft WPR/WPA for ETW CPU, scheduling, allocation and I/O investigation [S19]. ETL files may include sensitive paths or system activity: keep originals local, commit only aggregate results and synthetic reproductions.

## Scenario matrix

| Scenario | Procedure | Capture / question |
| --- | --- | --- |
| Cold start | Fresh OS session; fixed cache state; repeated launches | Launch-to-interactive with external timing; disk reads; all processes |
| Warm start | Relaunch same build/cache; distinguish resident launcher | Startup percentiles separately from cold runs |
| Idle visible/minimized | 5-minute settle, then 10-minute capture in each state | CPU wakeups, private bytes, private working set, GPU memory/power |
| Channel navigation | Same 20 channels and fixed history per channel | Input-to-paint p50/p95/p99; retained state after leaving |
| History stress | Large synthetic history; scroll and resize | Layout work, frame pacing, decoded media retention |
| Search/compose | Fixed search terms, long message, IME/emoji | Input latency, composition correctness and cancellation |
| Attachments | Fixed image dimensions/count; open/close viewer | Decode CPU, GPU allocations, cache return toward plateau |
| Voice | 30-minute controlled call; mute/deafen/PTT; switch devices | Audio glitches, callback timing, DAVE/device recovery |
| Video/share | Fixed resolution/FPS/codec where exposed | Encoder/GPU load, capture latency; deferred feature baseline |
| Network disruption | Disconnect/reconnect own test machine; controlled lab loss | Session recovery, duplicate messages, gaps and backoff |
| Long soak | Repeat workload for 2 hours then return to idle | Private-byte slope and retained resource plateau |
| Sleep/resume | Suspend with app open, resume and change network | Stale sockets, DNS changes, audio recovery |

## Practical first capture

```powershell
# Aggregate named processes; does not read credentials, content or command lines.
.\tools\Measure-Client.ps1 -ProcessName Discord -Scenario idle-visible -DurationSeconds 600
.\tools\Measure-Client.ps1 -ProcessName rillwire-desktop -Scenario idle-visible -DurationSeconds 600

# Optional elevated WPR session, after reviewing local privacy implications.
wpr -profiles
wpr -start GeneralProfile -filemode
# Perform the short documented workload.
wpr -stop .\captures\discord-navigation.etl
```

Create `captures` before invoking WPR. Check active WPR sessions before starting; never cancel someone else's trace. Use a custom memory/GPU profile only after the first trace identifies the bottleneck. WPR profile names and coverage should be checked on the installed version. [S19]

## API/behavior investigation sequence

1. Build a capability inventory from official docs and record unresolved UI behaviors.
2. Use synthetic fixture replay to verify local decoding and recovery semantics.
3. If a bot lab is selected, register an app and test only its granted permissions in the controlled server; record method/path templates and timing, not tokens or full content.
4. Discuss standalone-client eligibility and required scopes with Discord; draft questions are in `PARTNER-QUESTIONS.md`.
5. Resolve the access/terms decision before deeper protocol reverse engineering. Do not equate UI observation, public API use, binary decompilation and decrypted traffic capture: they are distinct methods with different requirements.
6. Keep a provenance ledger: observation, build/version, scenario, primary source, confidence, redacted reproduction and next experiment. A single trace is a hypothesis, not a protocol guarantee.

The current tools do not extract user tokens, inject into Discord, bypass certificate pinning, enumerate private APIs or collect other people's messages. Those operations are unnecessary for the initial performance baseline. This phase can produce useful engineering evidence without them.
