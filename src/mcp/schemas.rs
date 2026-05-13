use serde_json::{json, Value};

pub(super) const UNIFI_ACTIONS: &[&str] = &[
    "clients", "devices", "wlans", "health", "alarms", "events", "sysinfo", "me", "help",
];

pub(super) fn tool_definitions() -> Vec<Value> {
    vec![json!({
        "name": "unifi",
        "description": "Query a UniFi network controller via REST API (read-only). Use action=help for documentation.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Operation to perform.",
                    "enum": UNIFI_ACTIONS
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return (events only).",
                    "minimum": 1
                }
            },
            "required": ["action"]
        }
    })]
}
