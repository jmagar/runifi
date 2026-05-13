# AGENTS.md — rustifi

Rust binary MCP server for UniFi network controllers. Read-only REST API bridge exposing 8 actions.

## Essential Commands

```bash
cargo check                              # type-check (must pass before any PR)
cargo test                               # run all tests (no network required)
cargo run --bin unifi -- --help          # CLI help
cargo run --bin unifi -- health --json   # test a live action
cargo run --bin unifi                    # HTTP MCP server on :7474
cargo run --bin unifi -- mcp             # stdio MCP transport
```

## Architecture (strict layering)

```
UnifiClient (src/unifi.rs)   — HTTP only, no logic
    ↓
UnifiService (src/app.rs)    — all business logic
    ↓
tools.rs / cli.rs            — thin shims (parse + dispatch + format)
```

Never add business logic to tools.rs, cli.rs, or main.rs.

## Adding an Action

1. `src/unifi.rs` — add REST method
2. `src/app.rs` — delegate
3. `src/mcp/tools.rs` — match arm in dispatch()
4. `src/mcp/schemas.rs` — add to UNIFI_ACTIONS enum + schema
5. `src/mcp/rmcp_server.rs` — add to READ_ONLY_ACTIONS
6. `src/cli.rs` — CliCommand variant + parse + dispatch + formatter
7. Update HELP_TEXT in tools.rs and print_usage() in main.rs

## Key Files

- `src/unifi.rs` — UnifiClient, site_path() helper, UDM vs legacy path logic
- `src/config.rs` — UnifiConfig fields and env var names
- `src/mcp/schemas.rs` — JSON schema served to MCP clients
- `.env.example` — all supported env vars with documentation

## Environment

Minimum required:
```
UNIFI_URL=https://unifi.local
UNIFI_API_KEY=<your-api-key>
```

TLS: always set `UNIFI_SKIP_TLS_VERIFY=true` for self-signed controller certs.

## Tests

Tests in `tests/` do not require a live UniFi controller:
- `tool_dispatch.rs` — MCP dispatch (help, unknown action, missing action)
- `cli_parse.rs` — CLI argument parsing for all 8 commands
