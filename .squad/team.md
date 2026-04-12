# Squad Team

> Naked Time — Pomodoro-based time management productivity app

## Coordinator

| Name | Role | Notes |
|------|------|-------|
| Squad | Coordinator | Routes work, enforces handoffs and reviewer gates. |

## Members

| Name | Role | Charter | Status |
|------|------|---------|--------|
| Ripley | Lead | `.squad/agents/ripley/charter.md` | 🏗️ Lead |
| Dallas | Systems Dev | `.squad/agents/dallas/charter.md` | 🔧 Systems |
| Parker | CLI/TUI Dev | `.squad/agents/parker/charter.md` | ⚛️ CLI/TUI |
| Lambert | Tester | `.squad/agents/lambert/charter.md` | 🧪 Tester |
| Scribe | Scribe | `.squad/agents/scribe/charter.md` | 📋 Silent |
| Ralph | Work Monitor | `.squad/agents/ralph/charter.md` | 🔄 Monitor |

## Project Context

- **Owner:** Michael Collins
- **Project:** Naked Time — Pomodoro-based time management productivity application
- **Stack:** Rust (CLI/TUI + background service), cross-platform (Windows, macOS, Linux)
- **Created:** 2026-04-12
- **Description:** Single-user desktop app. CLI/TUI frontend + user-scoped background service with APIs. Future: Tauri or native apps.
- **Targets:** Windows (x86/x64/ARM64), macOS (x64/ARM64), Linux (x64/ARM64, glibc + musl)
