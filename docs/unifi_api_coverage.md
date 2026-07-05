# UniFi API Coverage

## Sources

- Official Network API: `data/unifi_official_network_v10_3_58.json`
- Internal endpoint models: `data/unifi_internal_endpoint_models.json`

## API Families

- `official`: documented Network Integration API under `/proxy/network/integration/v1`.
- `internal`: Network controller APIs under `/proxy/network/api/s/{site}` and `/proxy/network/v2/api/site/{site}`.
- `hybrid`: convenience actions that use internal actions by default and switch to official API when `siteId` or `prefer="official"` is supplied.

## Initial Coverage

- Official Network operations targeted: 78.
- Internal Network reference rows accounted: 12; live-verified runtime capabilities: 12.
- Existing live-verified rustifi actions preserved: clients, devices, wlans, health, alarms, sysinfo, me.

## Implementation Status

| Action | Family | Endpoint | Status |
|---|---|---|---|
| `official_*` | official | `/proxy/network/integration/v1/...` | implemented by generic dispatcher |
| `clients` | internal | `GET /stat/sta` | preserved |
| `devices` | internal | `GET /stat/device` | preserved |
| `wlans` | internal | `GET /rest/wlanconf` | preserved |
| `health` | internal | `GET /stat/health` | preserved |
| `alarms` | internal | `GET /rest/alarm` | preserved |
| `sysinfo` | internal | `GET /stat/sysinfo` | preserved |
| `me` | internal | `GET /proxy/network/api/self` | preserved |
| `internal_list_alarms` | internal | `GET /rest/alarm` | generic internal dispatcher |
| `internal_get_network_health` | internal | `GET /stat/health` | generic internal dispatcher |
| `internal_list_networks` | internal | `GET /rest/networkconf` | generic internal dispatcher |
| `internal_list_port_forwards` | internal | `GET /rest/portforward` | generic internal dispatcher |
| `internal_trigger_rf_scan` | internal | `POST /cmd/devmgr` | admin-authorized generic dispatcher |
| `list_clients` | hybrid | official clients or `clients` | implemented |
| `list_devices` | hybrid | official devices or `devices` | implemented |
| `list_networks` | hybrid | official networks or `internal_list_networks` | implemented |
| `list_wifi` | hybrid | official WiFi or `wlans` | implemented |
| `get_system_info` | hybrid | official info or `sysinfo` | implemented |

Official parity means every operation in `data/unifi_official_network_v10_3_58.json` is registered as an action, has a valid path template, has an auth scope, and is either contract-verified or safe-live verified. Contract verification is the CI-safe floor; live probing is an operator action.

The internal runtime surface is model-backed by `data/unifi_internal_endpoint_models.json` and exposes only rows with `runtime=true`. Non-runtime rows remain in the model inventory for accounting instead of being deleted to make verification green.

## Endpoint Verification

Run contract verification without network access:

```bash
cargo run -p xtask -- verify-api-endpoints --mode contract
```

Run live read probes against a controller with:

```bash
UNIFI_URL=https://<gateway> \
UNIFI_API_KEY=<network-api-key> \
UNIFI_SITE=default \
UNIFI_SITE_ID=<official-site-uuid> \
UNIFI_SKIP_TLS_VERIFY=true \
cargo run -p xtask -- verify-api-endpoints --mode safe_live
```

The verifier writes local reports under `target/unifi_verification/`; these reports must not be committed.

Result interpretation:

- `live_ok`: endpoint returned a 2xx response in live mode.
- `contract_ok`: endpoint is registered, path-valid, auth-scoped, and safe by policy in contract mode.
- `requires_fixture`: endpoint needs a concrete object ID or fixture before live probing.
- `auth_failed`: API key was rejected or lacks permission.
- `server_error`: request failed or controller returned 5xx.
- `skipped`: endpoint was disabled by mode or request budget.

`mutating_live` is reserved for disposable or controlled sites. It is never the default.
