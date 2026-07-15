use unifi::{UnifiClient, UnifiConfig};

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
