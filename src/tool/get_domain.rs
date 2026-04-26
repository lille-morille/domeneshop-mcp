use rmcp::ErrorData as McpError;
use serde::Deserialize;
use serde_json::Value;

use crate::client::ApiClient;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain name (e.g. `example.com`).
    pub domain: String,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    let id = client.domain_id_by_name(&p.domain).await?;
    let body = client.fetch_text(&format!("/domains/{id}")).await?;
    let mut domain: Value = serde_json::from_str(&body)
        .map_err(|e| McpError::internal_error(format!("parsing /domains/{id}: {e}"), None))?;
    if let Some(obj) = domain.as_object_mut() {
        obj.remove("registrant");
        obj.remove("renew");
        obj.remove("services");
    }
    serde_json::to_string(&domain)
        .map_err(|e| McpError::internal_error(format!("serializing /domains/{id}: {e}"), None))
}
