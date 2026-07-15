use unifi::{ActionDispatcher, ActionRequest, UnifiClient, UnifiConfig, UnifiService};

#[test]
fn core_crate_builds_client_without_app_runtime() {
    let client = UnifiClient::new(&UnifiConfig {
        url: "https://gateway.local/".to_string(),
        api_key: "test-key".to_string(),
        site: "default".to_string(),
        skip_tls_verify: true,
        legacy: false,
    })
    .expect("client should build without MCP app runtime");

    let config = client.config();
    assert_eq!(config.url, "https://gateway.local");
    assert_eq!(config.api_key, "test-key");
    assert_eq!(config.site, "default");
}

#[tokio::test]
async fn core_crate_owns_action_dispatch_and_capabilities() {
    let cap = unifi::capabilities::find_capability("clients").expect("clients capability");
    assert_eq!(cap.path.as_deref(), Some("/stat/sta"));

    let dispatcher = ActionDispatcher::new_for_test(UnifiConfig {
        url: "https://gateway.local".to_string(),
        api_key: "test-key".to_string(),
        site: "default".to_string(),
        skip_tls_verify: true,
        legacy: false,
    });

    let result = dispatcher
        .execute(ActionRequest {
            action: "list_clients".to_string(),
            params: serde_json::json!({}),
        })
        .await;

    let message = result.unwrap_err().to_string();
    assert!(message.contains("/proxy/network/api/s/default/stat/sta"));
}

#[tokio::test]
async fn core_crate_owns_service_facade() {
    let client = UnifiClient::new(&UnifiConfig {
        url: "https://gateway.local".to_string(),
        api_key: "test-key".to_string(),
        site: "default".to_string(),
        skip_tls_verify: true,
        legacy: false,
    })
    .expect("client should build");
    let service = UnifiService::new(client);

    let result = service
        .execute(ActionRequest {
            action: "list_clients".to_string(),
            params: serde_json::json!({}),
        })
        .await;

    let message = result.unwrap_err().to_string();
    assert!(message.contains("/proxy/network/api/s/default/stat/sta"));
}
