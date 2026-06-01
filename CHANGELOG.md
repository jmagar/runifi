# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] — 2026-06-01

### Changed

- Plugin `SessionStart`/`ConfigChange` hooks now call `${CLAUDE_PLUGIN_ROOT}/bin/runifi setup plugin-hook` directly instead of going through the `plugin-setup.sh` shell wrapper. The env-var mapping the script performed (`CLAUDE_PLUGIN_OPTION_*` → `UNIFI_*`, plus `CLAUDE_PLUGIN_DATA` → `UNIFI_MCP_HOME`) now lives in `apply_plugin_options()` in `src/setup.rs`, applied at the top of the plugin-hook path. The script's `.env`-fallback was dropped (immaterial: the binary never persists option values to `.env` and the setup checks read live process env).

### Removed

- `plugins/unifi/hooks/plugin-setup.sh` — the wrapper was a pure env-mapping middleman now handled by the binary's `setup plugin-hook` command.

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
