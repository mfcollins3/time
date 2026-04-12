# Project Context

- **Owner:** Michael Collins
- **Project:** Naked Time — a Pomodoro-based time management productivity application
- **Stack:** Go (CLI/TUI + background service), Python (AI agent), cross-platform (Windows, macOS, Linux)
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

### 2026-04-12: Implementation Language Changed to Go
- **Why:** Ripley completed honest re-evaluation after Michael relaxed the language uniformity constraint
- **Decision:** All core components (timed, time-cli, time-mcp, time-core) now recommended in Go instead of Rust
- **Why Go:** Goroutines for daemon concurrency, native Windows UDS, simple cross-compilation (GOOS/GOARCH), gRPC alignment, platform service lifecycle support
- **AI agent:** Remains Python (LangChain Deep Agents)
- **Product:** Now officially polyglot
- **Key ecosystem:** google.golang.org/grpc, modernc.org/sqlite, bubbletea, kardianos/service, BurntSushi/toml

### 2026-04-12: ADR-0001 System Architecture Accepted
- **Status:** Accepted and merged to main (commit 613c108). ADR-0001 is now the authoritative architecture baseline.
- **Impact:** Go-based core components, UDS IPC, gRPC APIs, SQLite, plugin system, and cross-platform service registration are locked in. Team cleared for implementation planning.

### 2026-04-12: v0.0.1 Product Specs and Version Target Finalized
- **Version:** 0.0.1 (not 0.1.0) — per Michael's directive
- **Duration warnings:** Silently accept all durations — no warnings for < 1 min or > 2 hours
- **Config file:** Optional with sensible defaults (no auto-creation of config.toml)
- **CLI framework:** Use Cobra for commands, Viper for configuration (Michael's directive)
- **Timestamps:** Store in UTC as ISO 8601 TEXT; parse user input as local time (Michael's directive)
- **Impact on Parker:** Simplify validation logic (no duration bounds), provide sensible defaults for all config keys, use Cobra + Viper for command structure and config loading
- **Key reference:** `.squad/decisions.md` Decisions #4, #6, #7

