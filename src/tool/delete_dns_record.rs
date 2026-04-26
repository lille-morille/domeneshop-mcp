use rmcp::ErrorData as McpError;
use serde::Deserialize;

use crate::client::ApiClient;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain name (e.g. `example.com`).
    pub domain: String,
    /// ID of the DNS record to delete.
    pub record_id: i64,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    let domain_id = client.domain_id_by_name(&p.domain).await?;
    client
        .generated
        .delete_domains_domain_id_dns_record_id(domain_id, p.record_id)
        .await
        .map_err(|e| McpError::internal_error(format!("Domeneshop API error: {e}"), None))?;
    Ok(format!(
        "Deleted DNS record {} from domain {}",
        p.record_id, p.domain
    ))
}
