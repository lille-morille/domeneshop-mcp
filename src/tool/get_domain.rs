use rmcp::ErrorData as McpError;
use serde::Deserialize;

use crate::client::ApiClient;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain ID (from `list_domains`).
    pub domain_id: i64,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    client.fetch_text(&format!("/domains/{}", p.domain_id)).await
}
