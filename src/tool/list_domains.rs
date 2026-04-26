use rmcp::ErrorData as McpError;
use serde_json::Value;

use crate::client::ApiClient;

pub async fn handle(client: &ApiClient) -> Result<String, McpError> {
    let body = client.fetch_text("/domains").await?;
    let domains: Vec<Value> = serde_json::from_str(&body)
        .map_err(|e| McpError::internal_error(format!("parsing /domains: {e}"), None))?;
    let names: Vec<&str> = domains
        .iter()
        .filter_map(|d| d.get("domain").and_then(Value::as_str))
        .collect();
    serde_json::to_string(&names)
        .map_err(|e| McpError::internal_error(format!("serializing /domains summary: {e}"), None))
}
