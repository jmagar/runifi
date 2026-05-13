# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-05-13

### Added

- Initial release of `rustifi` — UniFi MCP server bridging Claude to Ubiquiti network controllers
- MCP server with action-based tool dispatch (`unifi` tool, `action` parameter)
- Actions: `clients`, `devices`, `wlans`, `health`, `alarms`, `events`, `sysinfo`, `me`, `help`
- CLI thin shim with human-readable formatters and `--json` passthrough
- Bearer token + Google OAuth authentication via `lab-auth`
- Streamable HTTP transport on port 7474 + stdio transport
- Self-signed TLS support (`UNIFI_SKIP_TLS_VERIFY=true` default)
- Docker deployment with `ghcr.io/jmagar/rustifi` image
- Claude Code plugin with userConfig
- `entrypoint.sh` with permission setup and runtime validation
- Git LFS for pre-built plugin binaries in `bin/`
- nextest configuration with `ci` profile
- taplo TOML formatter configuration
- lefthook pre-commit hooks (diff check, TOML format, env guard)
- GitHub Actions: CI, Docker publish, release workflows
- xtask crate with `dist`, `ci`, `symlink-docs`, `check-env` commands
