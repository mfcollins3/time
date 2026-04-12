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

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
