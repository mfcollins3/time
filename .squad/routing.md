# Work Routing

How to decide who handles what.

## Routing Table

| Work Type | Route To | Examples |
|-----------|----------|----------|
| Architecture & design | Ripley | System architecture, ADRs, Rust vs Go, cross-platform strategy |
| Code review | Ripley | Review PRs, check quality, enforce architecture decisions |
| Scope & priorities | Ripley | What to build next, trade-offs, feature scope |
| Background service | Dallas | Daemon lifecycle, service APIs, data persistence, IPC |
| API design & impl | Dallas | REST/gRPC endpoints, request/response types, serialization |
| Data layer | Dallas | Storage, migrations, state management |
| CLI application | Parker | Argument parsing, subcommands, CLI structure |
| Terminal UI (TUI) | Parker | Ratatui/crossterm, layouts, widgets, user interaction |
| UX & interaction design | Parker | User flows, terminal UX, discoverability |
| Testing | Lambert | Unit tests, integration tests, property-based tests, edge cases |
| CI/CD test config | Lambert | Cross-platform test matrix, CI pipeline |
| Quality assurance | Lambert | Bug hunting, regression prevention, coverage |
| Session logging | Scribe | Automatic — never needs routing |

## Issue Routing

| Label | Action | Who |
|-------|--------|-----|
| `squad` | Triage: analyze issue, assign `squad:{member}` label | Ripley |
| `squad:ripley` | Architecture, design, review tasks | Ripley |
| `squad:dallas` | Background service, API, data layer tasks | Dallas |
| `squad:parker` | CLI, TUI, user interaction tasks | Parker |
| `squad:lambert` | Testing, QA, quality tasks | Lambert |

### How Issue Assignment Works

1. When a GitHub issue gets the `squad` label, **Ripley** triages it — analyzing content, assigning the right `squad:{member}` label, and commenting with triage notes.
2. When a `squad:{member}` label is applied, that member picks up the issue in their next session.
3. Members can reassign by removing their label and adding another member's label.
4. The `squad` label is the "inbox" — untriaged issues waiting for Ripley's review.

## Rules

1. **Eager by default** — spawn all agents who could usefully start work, including anticipatory downstream work.
2. **Scribe always runs** after substantial work, always as `mode: "background"`. Never blocks.
3. **Quick facts → coordinator answers directly.** Don't spawn an agent for "what port does the server run on?"
4. **When two agents could handle it**, pick the one whose domain is the primary concern.
5. **"Team, ..." → fan-out.** Spawn all relevant agents in parallel as `mode: "background"`.
6. **Anticipate downstream work.** If a feature is being built, spawn the tester to write test cases from requirements simultaneously.
7. **Issue-labeled work** — when a `squad:{member}` label is applied to an issue, route to that member. Ripley handles all `squad` (base label) triage.
