use rmcp::{
    ErrorData as McpError,
    model::{RawResource, RawResourceTemplate, ResourceContents},
};

use crate::client::ApiClient;

pub const URI_TEMPLATE: &str = "domeneshop://domains/{domainId}/dns";

pub fn uri_for(domain_id: i64) -> String {
    format!("domeneshop://domains/{domain_id}/dns")
}

pub fn template() -> RawResourceTemplate {
    RawResourceTemplate {
        uri_template: URI_TEMPLATE.into(),
        name: "dns-records".into(),
        title: Some("DNS records for a domain".into()),
        description: Some(
            "JSON array of DNS records for the given domain id. Substitute `{domainId}` \
             with the id from the `domeneshop://domains` resource."
                .into(),
        ),
        mime_type: Some("application/json".into()),
        icons: None,
    }
}

pub fn descriptor_for(domain_id: i64, display_name: Option<&str>) -> RawResource {
    let uri = uri_for(domain_id);
    let label = display_name.unwrap_or("domain");
    let mut res = RawResource::new(uri, format!("dns:{label}"));
    res.title = Some(format!("DNS records for {label}"));
    res.description = Some(format!("All DNS records on domain id {domain_id}"));
    res.mime_type = Some("application/json".into());
    res
}

/// Parse a URI like `domeneshop://domains/42/dns` and return the domain id.
pub fn parse_uri(uri: &str) -> Option<i64> {
    let rest = uri.strip_prefix("domeneshop://domains/")?;
    let (id, tail) = rest.split_once('/')?;
    if tail != "dns" {
        return None;
    }
    id.parse().ok()
}

pub async fn read(client: &ApiClient, domain_id: i64) -> Result<Vec<ResourceContents>, McpError> {
    let body = client
        .fetch_text(&format!("/domains/{domain_id}/dns"))
        .await?;
    Ok(vec![ResourceContents::TextResourceContents {
        uri: uri_for(domain_id),
        mime_type: Some("application/json".into()),
        text: body,
        meta: None,
    }])
}
