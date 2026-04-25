use rmcp::ErrorData as McpError;
use serde::Deserialize;

use crate::api::types::DnsRecord;
use crate::client::ApiClient;

use super::dns::{RecordType, build_dns_record};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct Params {
    /// Domain ID (from `list_domains`).
    pub domain_id: i64,
    /// DNS record type.
    #[serde(rename = "type")]
    pub record_type: RecordType,
    /// Host/subdomain. Use "@" for apex, "*" for wildcard, or a name like "www".
    pub host: String,
    /// TTL in seconds. Must be a multiple of 60, between 60 and 604800. Defaults to 3600.
    #[serde(default)]
    pub ttl: Option<i64>,
    /// Target value. IPv4 for A, IPv6 for AAAA, hostname for CNAME/MX/SRV,
    /// free text for TXT, or the hex hash for TLSA.
    pub data: String,
    /// MX/SRV priority (preference). Required for those types.
    #[serde(default)]
    pub priority: Option<i16>,
    /// SRV weight. Required for SRV only.
    #[serde(default)]
    pub weight: Option<i16>,
    /// SRV port. Required for SRV only.
    #[serde(default)]
    pub port: Option<i16>,
    /// TLSA usage field. Required for TLSA only.
    #[serde(default)]
    pub usage: Option<i16>,
    /// TLSA selector. Required for TLSA only.
    #[serde(default)]
    pub selector: Option<i16>,
    /// TLSA matching type (aka dtype). Required for TLSA only.
    #[serde(default)]
    pub matching_type: Option<i16>,
}

pub async fn handle(client: &ApiClient, p: Params) -> Result<String, McpError> {
    let domain_id = p.domain_id;
    let record_type = p.record_type;
    let host = p.host.clone();
    let body: DnsRecord = build_dns_record(p, 0)?;

    let resp = client
        .generated
        .post_domains_domain_id_dns(domain_id, &body)
        .await
        .map_err(|e| McpError::internal_error(format!("Domeneshop API error: {e}"), None))?;

    let id = resp.into_inner().id;
    Ok(format!(
        "Created {record_type:?} record for host {host:?} on domain {domain_id} (id: {})",
        id.map_or_else(|| "unknown".into(), |v| v.to_string())
    ))
}
