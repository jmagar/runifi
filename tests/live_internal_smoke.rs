use serde_json::json;

use unifi::{ActionDispatcher, ActionRequest};
use unifi_rmcp::config::Config;

#[tokio::test]
#[ignore]
async fn internal_smoke_actions() {
    unifi_rmcp::config::load_dotenv();
    let config = Config::load().expect("config should load");
    let dispatcher = ActionDispatcher::new(config.unifi);
    for action in ["clients", "devices", "health", "me"] {
        dispatcher
            .execute(ActionRequest {
                action: action.to_string(),
                params: json!({}),
            })
            .await
            .unwrap_or_else(|error| panic!("{action} failed: {error}"));
    }
}
