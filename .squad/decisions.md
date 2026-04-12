# Squad Decisions

## Active Decisions

### 1. System Architecture (ADR-0001)
**Author:** Ripley  
**Date:** 2026-04-12  
**Status:** Proposed

- **Daemon (`timed`)** is user-scoped background service, auto-started via launchd / systemd / Task Scheduler
- **IPC is Unix domain sockets on ALL platforms** including Windows 11 (named pipes rejected)
- **gRPC (tonic)** is primary protocol, HTTP/2 over single UDS
- **SQLite** bundled (WAL mode) at `~/.mfcollins3/time/data/time.db`
- **Plugin architecture** — child processes via gRPC over per-plugin UDS
- **AI agent** is Python child process (LangChain Deep Agents), not embedded
- **MCP server** is separate binary (`time-mcp`) bridging MCP to daemon gRPC
- **Implementation language:** Go (all core components: timed, time-cli, time-mcp, time-core). Python for AI agent only.
- **9 build targets** defined (Windows x86/x64/ARM64, macOS x64/ARM64, Linux x64/ARM64 glibc/musl)
- **File:** `docs/adrs/0001-system-architecture.md`

### 2. Product Polyglot Constraint Relaxed
**Author:** Michael Collins  
**Date:** 2026-04-12T02:34:37Z  
**Status:** Active

Product can be heterogeneous. Use the best tool for the job. Go is acceptable for timed daemon if justified.

### 3. Language Re-evaluation — Go Replaces Rust for Core Components
**Author:** Ripley  
**Date:** 2026-04-12  
**Status:** Active (ADR-0001 Section 7+ revised)

**Decision:** All core components (timed, time-cli, time-mcp, time-core) now recommended in Go. AI agent remains Python.

**Why Go wins:**
- Goroutines right-sized for modest daemon concurrency (timer + gRPC)
- `net.Listen("unix", path)` works on Windows natively
- `GOOS/GOARCH` eliminates Docker/QEMU for 9-target cross-compilation
- `google.golang.org/grpc` is reference implementation
- `kardianos/service` handles cross-platform service lifecycle
- Lower contributor onboarding curve

**Why Rust didn't win:**
- Resource efficiency difference (2 MB vs 8 MB RSS) is noise for timer daemon
- Compile-time data race prevention valuable but not decisive for straightforward goroutine patterns
- Tokio's power exceeds workload demands

**Key Go ecosystem:**
- `modernc.org/sqlite` (pure Go, no CGO)
- `bubbletea`/Charm (TUI)
- `BurntSushi/toml` (config)
- Module layout: `cmd/` (binaries), `internal/` (private), `pkg/proto/` (generated)

### 4. CLI Framework and Config Preferences
**Author:** Michael Collins  
**Date:** 2026-04-12T05:32:00Z  
**Status:** Active

- **CLI framework:** Use Cobra (`cobra.dev`) for CLI commands
- **Configuration management:** Use Viper (`spf13/viper`) for application configuration management
- **Timestamps in database:** Store in UTC as ISO 8601 TEXT (e.g., `2026-04-11T17:00:00Z`)
- **User timestamp input:** When users provide timestamps without timezone offset or with `Z`, assume local time zone and convert to UTC

**Rationale:** Cobra provides ergonomic command structure. Viper integrates with Cobra flags and supports config file loading (TOML), environment variable binding with `NAKEDTIME_` prefix. UTC storage is industry standard. Local-timezone input default matches user expectations.

### 5. Technology Evaluation — GORM vs Direct SQL
**Author:** Ripley  
**Date:** 2026-04-12  
**Status:** Decision

**Decision:** Stay with direct SQL. GORM does not earn its place in this codebase.

**Reasoning:**
- Schema is simple (≤8 tables), CRUD with date range filters — GORM is overkill
- AutoMigrate is add-only only; hand-rolled numbered, forward-only migrations are explicit and sufficient
- `internal/core/` package maintains zero external dependencies by design
- `PomodoroStore` interface provides necessary abstraction; GORM would add indirection without benefit
- Direct SQL is more debuggable and transparent
- When to use GORM: 30+ tables with complex relationships, multiple database backends, or existing team expertise — none apply to Naked Time

**Impact:** Confirms existing plan. Codebase uses direct SQL via `database/sql` + `modernc.org/sqlite` with `PomodoroStore` interface for persistence abstraction.

### 6. UTC Timestamp Storage and Local-Timezone Input Convention
**Author:** Ripley  
**Date:** 2026-04-12  
**Status:** Active

- **Database storage:** All timestamps as ISO 8601 UTC TEXT (e.g., `2026-04-11T17:00:00Z`)
- **User display:** Convert stored UTC to local timezone for display
- **Input parsing:** Explicit offset → convert to UTC; `Z` suffix → UTC; no offset → assume local timezone, convert to UTC
- **Scope:** All timestamp columns in all tables, all CLI input parsing, all display formatting

**Rationale:** UTC storage is universal convention with no credible alternative. Local-timezone input default is UX-driven: users typing `10:00:00` mean 10 AM in their timezone, not UTC.

### 7. Cobra + Viper Bootstrap and Initial Version
**Author:** Ripley  
**Date:** 2026-04-12  
**Status:** Active

**Decisions:**
1. **Cobra for CLI commands** — per Michael's directive (non-negotiable). Dependency: `github.com/spf13/cobra`
2. **Viper for configuration** — per Michael's directive ("if it makes sense") — it does. Viper provides config file loading (TOML), environment variable binding (`NAKEDTIME_` prefix), Cobra flag integration. Supersedes ADR-0001's mention of `BurntSushi/toml`; Viper uses `pelletier/go-toml` internally. Dependency: `github.com/spf13/viper`
3. **CLI binary name:** `nakedtime` (not `time-cli`). Product name is the command name. Source directory: `cmd/nakedtime/`. ADR-0001 was illustrative on naming, not prescriptive.
4. **Direct SQLite access for CLI bootstrap:** ADR-0001 says daemon owns database. Daemon not yet built. CLI talks directly to SQLite behind `PomodoroStore` interface. When daemon is built, `GRPCStore` implementation replaces `SQLiteStore` in CLI wiring — no command code changes. Pragmatic bootstrapping, not architecture deviation.

**Version info (from Michael's directive 2026-04-12T05:55:00Z):**
- Initial version: **0.0.1** (not 0.1.0)
- Duration warnings: silently accept all durations — no warnings for < 1 min or > 2 hours
- Config file: optional with sensible defaults (no auto-creation of config.toml)

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
