use serde_json::{json, Value};

use super::AppState;

/// Thin shim — parse args, call service, return Value. No logic here.
pub(super) async fn execute_tool(
    state: &AppState,
    name: &str,
    args: Value,
) -> anyhow::Result<Value> {
    match name {
        "unifi" => dispatch(state, args).await,
        _ => Err(anyhow::anyhow!("unknown tool: {name}")),
    }
}

async fn dispatch(state: &AppState, args: Value) -> anyhow::Result<Value> {
    let action =
        string_arg(&args, "action").ok_or_else(|| anyhow::anyhow!("action is required"))?;
    match action.as_str() {
        "clients" => state.service.clients().await,
        "devices" => state.service.devices().await,
        "wlans" => state.service.wlans().await,
        "health" => state.service.health().await,
        "alarms" => state.service.alarms().await,
        "events" => {
            let limit = usize_arg(&args, "limit")?;
            state.service.events(limit).await
        }
        "sysinfo" => state.service.sysinfo().await,
        "me" => state.service.me().await,
        "help" => Ok(json!({ "help": HELP_TEXT })),
        other => Err(anyhow::anyhow!(
            "unknown unifi action: {other}; use action=help for documentation"
        )),
    }
}

fn string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name).and_then(|v| v.as_str()).map(String::from)
}

fn usize_arg(args: &Value, name: &str) -> anyhow::Result<Option<usize>> {
    let Some(v) = args.get(name) else {
        return Ok(None);
    };
    v.as_u64()
        .map(|n| Some(n as usize))
        .ok_or_else(|| anyhow::anyhow!("`{name}` must be a non-negative integer"))
}

const HELP_TEXT: &str = r#"# unifi MCP Tool

Read-only access to UniFi network controllers via REST API.
Set the required `action` argument to select the operation.

## Network
- `clients`   — Connected wireless and wired clients
- `devices`   — Network devices: APs, switches, gateways
- `wlans`     — WiFi network configurations
- `health`    — Site health summary
- `alarms`    — Active alarms and alerts
- `events`    — Recent events (optional `limit` integer)
- `sysinfo`   — Controller system information
- `me`        — Authenticated user info

## Meta
- `help`      — This documentation
"#;
