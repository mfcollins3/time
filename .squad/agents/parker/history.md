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

