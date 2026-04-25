use rmcp::ErrorData as McpError;
use serde::Deserialize;
use serde_json::Value;

use crate::client::ApiClient;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain ID (from `list_domains`).
    pub domain_id: i64,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    let body = client
        .fetch_text(&format!("/domains/{}", p.domain_id))
        .await?;
    let mut domain: Value = serde_json::from_str(&body).map_err(|e| {
        McpError::internal_error(format!("parsing /domains/{}: {e}", p.domain_id), None)
    })?;
    if let Some(obj) = domain.as_object_mut() {
        obj.remove("registrant");
        obj.remove("renew");
        obj.remove("services");
    }
    serde_json::to_string(&domain).map_err(|e| {
        McpError::internal_error(format!("serializing /domains/{}: {e}", p.domain_id), None)
    })
}
