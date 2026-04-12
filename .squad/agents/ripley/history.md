# Project Context

- **Owner:** Michael Collins
- **Project:** Naked Time — a Pomodoro-based time management productivity application
- **Stack:** Rust (CLI/TUI + background service), cross-platform (Windows, macOS, Linux)
- **Created:** 2026-04-12

## Project Details

- Pomodoro Technique core: activities, focused iterations, short breaks, analysis
- Single user, desktop/laptop application
- CLI/TUI frontend communicating with a user-scoped background service via APIs
- Background service starts on login, runs persistently
- Future: possible Tauri web UI or native apps (SwiftUI, WinUI)
- Build targets: Windows (x86/x64/ARM64), macOS (x64/ARM64), Linux (x64/ARM64, glibc + musl)

## Learnings

<!-- Append new learnings below. Each entry is something lasting about the project. -->

### 2026-04-12: ADR-0001 System Architecture (Proposed)

- **ADR path:** `docs/adrs/0001-system-architecture.md`
- **Daemon binary:** `timed` — user-scoped, auto-starts on login
- **IPC:** Unix domain sockets (AF_UNIX) on all platforms including Windows 11
- **Primary protocol:** gRPC via `tonic` crate over UDS
- **Data store:** SQLite via `rusqlite` (bundled, WAL mode) at `~/.mfcollins3/time/data/time.db`
- **Config:** TOML at `~/.mfcollins3/time/config/config.toml`
- **Socket:** `~/.mfcollins3/time/timed.sock`
- **Platform service registration:** launchd (macOS), systemd user (Linux), Task Scheduler (Windows 11)
- **Plugin architecture:** Child processes communicate via gRPC over per-plugin UDS
- **AI agent:** Python child process using LangChain Deep Agents SDK, communicates via gRPC + HTTP (Responses API)
- **MCP server:** Separate `time-mcp` binary, bridges MCP JSON-RPC to daemon gRPC
- **Workspace crates:** timed, time-cli, time-mcp, time-core, time-proto, time-transport, time-platform
- **Build targets:** 9 total (see ADR for full matrix)
- **Windows UDS risk:** Validated that AF_UNIX is supported since Win10 build 17063; `uds_windows` crate provides Rust bridge. Early spike recommended.
- **User preference:** Michael prefers UDS over named pipes. Open to TCP fallback only if UDS proves unreliable on Windows.

### 2026-04-12: ADR-0001 Language Re-evaluation — Rust → Go

- **Trigger:** Michael relaxed the language uniformity constraint. Product may be polyglot.
- **Outcome:** Switched recommendation from Rust to Go for all core components (timed, time-cli, time-mcp, time-core). AI agent remains Python.
- **Key factors favoring Go:**
  - Goroutines are right-sized for this daemon's modest concurrency (timer + occasional gRPC). Tokio is overkill.
  - `net.Listen("unix", path)` works on Windows natively. No `uds_windows` shim needed.
  - `GOOS/GOARCH` eliminates Docker/QEMU/zigbuild for 9-target cross-compilation.
  - `google.golang.org/grpc` is the gRPC reference implementation.
  - `kardianos/service` handles cross-platform service lifecycle out of the box.
  - Lower learning curve for contributor onboarding.
- **Why Rust didn't win on merit alone:** Resource efficiency difference (2 MB vs 8 MB RSS) is noise for a timer daemon. Compile-time data race prevention is valuable but not decisive for straightforward goroutine patterns. Tokio's power exceeds what this workload demands.
- **Go ecosystem choices:** `modernc.org/sqlite` (pure Go, no CGO), `bubbletea`/Charm (TUI), `BurntSushi/toml` (config).
- **Module layout:** `cmd/` for binaries, `internal/` for private packages, `pkg/proto/` for generated code.
- **ADR status:** Still Proposed. Michael will accept when ready.

### 2026-04-12: ADR-0001 System Architecture Accepted
- **Status:** Accepted and merged to main (commit 613c108). ADR-0001 is now the authoritative architecture baseline.
- **Impact:** Core Go architecture, UDS IPC, gRPC APIs, SQLite, plugin system, and cross-platform service registration are locked in. Team cleared for implementation planning.
