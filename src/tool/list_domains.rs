use rmcp::ErrorData as McpError;
use serde_json::{Value, json};

use crate::client::ApiClient;

pub async fn handle(client: &ApiClient) -> Result<String, McpError> {
    let body = client.fetch_text("/domains").await?;
    let domains: Vec<Value> = serde_json::from_str(&body)
        .map_err(|e| McpError::internal_error(format!("parsing /domains: {e}"), None))?;
    let summary: Vec<_> = domains
        .iter()
        .map(|d| {
            json! ({
                "id": d.get("id"),
                "domain": d.get("domain"),
            })
        })
        .collect();
    serde_json::to_string(&summary)
        .map_err(|e| McpError::internal_error(format!("serializing /domains summary: {e}"), None))
}
