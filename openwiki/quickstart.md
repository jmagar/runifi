---
type: Reference
title: unifi-rmcp - UniFi MCP Server
description: unifi-rmcp exposes UniFi controller APIs through MCP and a CLI, with official/internal/hybrid actions, scope-aware HTTP auth modes, and setup hooks.
tags: [unifi, mcp, rust, cli, api]
timestamp: 2026-07-23T10:17:21Z
---

# unifi-rmcp - UniFi MCP Server

`unifi-rmcp` is a Rust MCP server and CLI that exposes UniFi Network controller
APIs through two interfaces:

- **CLI** (`runifi`) for direct operator commands.
- **MCP** (`unifi`) for AI clients.

It now supports official and internal UniFi API actions plus hybrid convenience
aliases, and enforces action-level authorization for MCP requests.

See [README.md](README.md) and [docs/unifi_api_coverage.md](docs/unifi_api_coverage.md).

## What unifi-rmcp does

- `official_*` actions target documented Network Integration paths under
  `/proxy/network/integration/v1/...`.
- `unifi_*` actions target internal site APIs (`/proxy/network/api/s/{site}/...` and
  `/proxy/network/v2/api/site/{site}/...`).
- Hybrid actions (for example `list_clients`, `list_devices`, `list_networks`) resolve
  to official or internal implementations.
- Action scope is checked (`unifi:read` / `unifi:admin`) before execution in HTTP MCP.

## Quick Start

### 1) Configure credentials

```bash
cp .env.example .env
# required
UNIFI_URL=https://your-controller.local
UNIFI_API_KEY=<api key>
```

### 2) Preflight checks

```bash
runifi doctor --json
runifi doctor
```

### 3) Run actions

```bash
runifi health --json
runifi clients --json
runifi devices --json
runifi official_list_clients --param siteId=<site uuid> --json
runifi list_clients --param siteId=<site uuid> --json
runifi setup check --json
```

### 4) Start MCP

- HTTP MCP (default `0.0.0.0:40030`):

  ```bash
  runifi
  ```

- stdio MCP (for local clients):

  ```bash
  runifi mcp
  ```

- npm launcher:

  ```bash
  npx -y unifi-rmcp health --json
  npx -y unifi-rmcp mcp
  ```

## Source-to-behavior map

- `src/main.rs`: command modes (`mcp`, `mcp` stdio, `doctor`, `setup`, version/help).
- `src/config.rs`: environment/config-file merge and load order.
- `src/cli.rs`: CLI parser/dispatcher.
- `src/mcp/tools.rs`: MCP shim dispatch (`action` + `params`).
- `src/mcp/schemas.rs`: MCP tool schema and action discovery.
- `src/mcp/rmcp_server.rs`: auth/scoping and request authorization.
- `src/setup.rs`: `setup check`, `setup repair`, `setup install`, `setup plugin-hook`.
- `crates/unifi/src/actions.rs`: action dispatch by family.
- `crates/unifi/src/capabilities.rs`: canonical action registry.

## Authentication and deployment model

HTTP MCP is guarded by one of these paths:

- Loopback no-auth (`127.0.0.1` / `localhost`) when explicitly local.
- Bearer token mode with `UNIFI_MCP_TOKEN`.
- OAuth mode with `UNIFI_MCP_AUTH_MODE=oauth` and Google credentials.
- `UNIFI_NOAUTH=true` when trust is delegated to an authenticated gateway.

`src/main.rs` enforces non-loopback binding protections for HTTP unless auth is
present.

Useful MCP variables:

- `UNIFI_MCP_HOST`, `UNIFI_MCP_PORT`, `UNIFI_MCP_TOKEN`, `UNIFI_MCP_NO_AUTH`
- `UNIFI_MCP_AUTH_MODE`, `UNIFI_MCP_PUBLIC_URL`
- `UNIFI_MCP_GOOGLE_CLIENT_ID`, `UNIFI_MCP_GOOGLE_CLIENT_SECRET`,
  `UNIFI_MCP_AUTH_ADMIN_EMAIL`

Controller variables:

- `UNIFI_URL`, `UNIFI_API_KEY`, `UNIFI_SITE`
- `UNIFI_SKIP_TLS_VERIFY` (common default is `true`)
- `UNIFI_LEGACY=true` for non-UDM controllers

## Setup commands and plugin integration

```bash
runifi setup check [--json]
runifi setup repair [--json]
runifi setup install
runifi setup plugin-hook [--no-repair] [--json]
```

`plugins/unifi/hooks/hooks.json` now invokes `runifi setup plugin-hook` directly
(no wrapper script).

`CLAUDE_PLUGIN_OPTION_*` values are mapped into `UNIFI_*` environment variables
before checks.

## Configuration references

- [docs/CONFIG.md](docs/CONFIG.md)
- [docs/OAUTH.md](docs/OAUTH.md)
- [src/config.rs](src/config.rs)
- [src/mcp/rmcp_server.rs](src/mcp/rmcp_server.rs)

## Verification and quality

```bash
cargo check
cargo test
cargo run -p xtask -- verify-api-endpoints --mode contract
```

For live verification:

```bash
UNIFI_URL=https://<gateway> \
UNIFI_API_KEY=<key> \
UNIFI_SITE=<site uuid> \
UNIFI_SKIP_TLS_VERIFY=true \
cargo run -p xtask -- verify-api-endpoints --mode safe_live
```

## Backlog

- No deferred documentation gaps.
