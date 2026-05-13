# rustifi

UniFi network MCP server — read-only REST API bridge for Ubiquiti UniFi controllers (UniFi OS / UDM and legacy).

Exposes connected clients, network devices, WiFi configurations, site health, alarms, events, and system info to MCP clients (Claude, Cursor, etc.) and as a CLI tool.

## UniFi API Overview

UniFi controllers expose a REST API. Modern UniFi OS (UDM/UDR) uses:

- Base URL: `https://<controller-ip>`
- Auth: `X-API-KEY` header (preferred, UniFi OS 3.x+)
- Site-scoped paths: `/proxy/network/api/s/{site}/...`

Legacy controllers (non-UDM, port 8443) use the same paths without the `/proxy/network` prefix. Set `UNIFI_LEGACY=true` for those.

**TLS note:** UniFi controllers use self-signed certificates by default. Always set `UNIFI_SKIP_TLS_VERIFY=true` unless you have installed a valid cert.

## Quickstart

```bash
# 1. Copy env template
cp .env.example .env
# Edit UNIFI_URL and UNIFI_API_KEY

# 2. Run CLI
source .env
cargo run --bin unifi -- health
cargo run --bin unifi -- clients

# 3. Run MCP HTTP server (port 7474)
cargo run --bin unifi

# 4. Run MCP stdio transport (for Claude Desktop, etc.)
cargo run --bin unifi -- mcp
```

## CLI Usage

```
unifi clients [--json]                Connected wireless and wired clients
unifi devices [--json]                Network devices: APs, switches, gateways
unifi wlans [--json]                  WiFi network configurations
unifi health [--json]                 Site health summary
unifi alarms [--json]                 Active alarms and alerts
unifi events [--limit N] [--json]     Recent events (optional limit)
unifi sysinfo [--json]                Controller system information
unifi me [--json]                     Authenticated user info
```

## MCP Actions

The `unifi` MCP tool accepts an `action` argument:

| Action    | Description                              |
|-----------|------------------------------------------|
| `clients` | Connected wireless and wired clients     |
| `devices` | Network devices: APs, switches, gateways |
| `wlans`   | WiFi network configurations              |
| `health`  | Site health summary                      |
| `alarms`  | Active alarms and alerts                 |
| `events`  | Recent events (optional `limit` integer) |
| `sysinfo` | Controller system information            |
| `me`      | Authenticated user info                  |
| `help`    | Tool documentation                       |

## Environment Variables

| Variable                    | Default       | Description                                      |
|-----------------------------|---------------|--------------------------------------------------|
| `UNIFI_URL`                 | —             | Controller base URL, e.g. `https://unifi.local` (required) |
| `UNIFI_API_KEY`             | —             | API key for `X-API-KEY` header (required)        |
| `UNIFI_SITE`                | `default`     | UniFi site name                                  |
| `UNIFI_SKIP_TLS_VERIFY`     | `true`        | Skip TLS cert check (needed for self-signed)     |
| `UNIFI_LEGACY`              | `false`       | Legacy controller mode (no `/proxy/network` prefix) |
| `UNIFI_MCP_HOST`            | `0.0.0.0`     | MCP server bind host                             |
| `UNIFI_MCP_PORT`            | `7474`        | MCP server bind port                             |
| `UNIFI_MCP_TOKEN`           | —             | Static bearer token for MCP auth                 |
| `UNIFI_MCP_NO_AUTH`         | `false`       | Disable MCP auth (loopback only)                 |
| `UNIFI_MCP_PUBLIC_URL`      | —             | Public URL for OAuth metadata                    |
| `UNIFI_MCP_AUTH_MODE`       | `bearer`      | Auth mode: `bearer` or `oauth`                   |
| `UNIFI_MCP_GOOGLE_CLIENT_ID`    | —         | Google OAuth client ID                           |
| `UNIFI_MCP_GOOGLE_CLIENT_SECRET`| —         | Google OAuth client secret                       |
| `UNIFI_MCP_AUTH_ADMIN_EMAIL`    | —         | Admin email for OAuth                            |
| `RUST_LOG`                  | `info`        | Log filter                                       |

## Generating an API Key

1. Log in to your UniFi OS dashboard
2. Go to Settings → Admins & Users → API Keys
3. Create a new key and copy it into `UNIFI_API_KEY`

## Architecture

```
src/
  unifi.rs       — HTTP REST client for UniFi API
  app.rs         — UnifiService: all business logic
  config.rs      — UnifiConfig + McpConfig
  mcp.rs         — AppState, AuthPolicy, module wiring
  mcp/tools.rs   — thin shim: parse args → call service → return Value
  mcp/schemas.rs — tool JSON schema definitions
  mcp/prompts.rs — MCP prompts (network_summary)
  mcp/rmcp_server.rs — rmcp ServerHandler impl
  mcp/routes.rs  — axum router with auth middleware
  cli.rs         — thin shim: parse args → call service → format/print
  lib.rs         — module declarations + test helpers
  main.rs        — dispatch: serve / mcp / cli
```
