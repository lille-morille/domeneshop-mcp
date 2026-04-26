use rmcp::ErrorData as McpError;
use serde::Deserialize;

use crate::api::types::DnsRecord;
use crate::client::ApiClient;

use super::dns::{RecordType, build_dns_record};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain name (e.g. `example.com`).
    pub domain: String,
    /// ID of the record to update.
    pub record_id: i64,
    /// DNS record type.
    #[serde(rename = "type")]
    pub record_type: RecordType,
    /// Host/subdomain.
    pub host: String,
    /// TTL in seconds. Multiple of 60, 60-604800. Defaults to 3600.
    #[serde(default)]
    pub ttl: Option<i64>,
    /// Target value (see `create_dns_record` for format per type).
    pub data: String,
    #[serde(default)]
    pub priority: Option<i16>,
    #[serde(default)]
    pub weight: Option<i16>,
    #[serde(default)]
    pub port: Option<i16>,
    #[serde(default)]
    pub usage: Option<i16>,
    #[serde(default)]
    pub selector: Option<i16>,
    #[serde(default)]
    pub matching_type: Option<i16>,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    let domain_id = client.domain_id_by_name(&p.domain).await?;
    let domain = p.domain.clone();
    let record_id = p.record_id;

    // The API's DnsRecord body carries an `id` in its BaseRecord slot.
    let create = create_from_update(p);
    let body: DnsRecord = build_dns_record(create, record_id)?;

    client
        .generated
        .put_domains_domain_id_dns_record_id(domain_id, record_id, &body)
        .await
        .map_err(|e| McpError::internal_error(format!("Domeneshop API error: {e}"), None))?;

    Ok(format!("Updated DNS record {record_id} on domain {domain}"))
}

fn create_from_update(p: Params) -> super::create_dns_record::Params {
    super::create_dns_record::Params {
        domain: p.domain,
        record_type: p.record_type,
        host: p.host,
        ttl: p.ttl,
        data: p.data,
        priority: p.priority,
        weight: p.weight,
        port: p.port,
        usage: p.usage,
        selector: p.selector,
        matching_type: p.matching_type,
    }
}
