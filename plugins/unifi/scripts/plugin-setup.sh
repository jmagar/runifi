#!/usr/bin/env bash
# SessionStart / ConfigChange hook for the UniFi plugin.
set -euo pipefail

binary="${RUSTIFI_MCP_BIN:-runifi}"

if ! command -v "${binary}" >/dev/null 2>&1; then
  printf 'unifi plugin setup: runifi is not installed or not on PATH.\n' >&2
  printf 'Install runifi separately, then run: runifi setup\n' >&2
  exit 0
fi

exec "${binary}" setup plugin-hook "$@"
