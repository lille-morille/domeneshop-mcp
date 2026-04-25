pub mod list_dns_records;
pub mod list_domains;

use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        Annotated, ListResourcesResult, ReadResourceRequestParams, ReadResourceResult, Resource,
        ResourceTemplate,
    },
    service::RequestContext,
};

use crate::{api, auth, client::ApiClient};

pub fn templates() -> Vec<ResourceTemplate> {
    vec![Annotated::new(list_dns_records::template(), None)]
}

pub async fn list(ctx: &RequestContext<RoleServer>) -> Result<ListResourcesResult, McpError> {
    let mut resources: Vec<Resource> =
        vec![Annotated::new(list_domains::descriptor(), None)];

    // Try to enumerate per-domain DNS resources; fall back silently if auth
    // is missing or the upstream call fails so initial discovery still works.
    if let Ok(client) = auth::api_client_for(ctx)
        && let Ok(domains) = fetch_domains(&client).await
    {
        for d in domains {
            let Some(id) = d.id else { continue };
            let descriptor = list_dns_records::descriptor_for(i64::from(id), d.domain.as_deref());
            resources.push(Annotated::new(descriptor, None));
        }
    }

    Ok(ListResourcesResult {
        resources,
        next_cursor: None,
        meta: None,
    })
}

pub async fn read(
    req: ReadResourceRequestParams,
    ctx: &RequestContext<RoleServer>,
) -> Result<ReadResourceResult, McpError> {
    let client = auth::api_client_for(ctx)?;
    let contents = if req.uri == list_domains::URI {
        list_domains::read(&client).await?
    } else if let Some(domain_id) = list_dns_records::parse_uri(&req.uri) {
        list_dns_records::read(&client, domain_id).await?
    } else {
        return Err(McpError::resource_not_found(
            format!("unknown resource: {}", req.uri),
            None,
        ));
    };
    Ok(ReadResourceResult { contents })
}

async fn fetch_domains(client: &ApiClient) -> Result<Vec<api::types::Domain>, McpError> {
    let domains = client
        .generated
        .get_domains(None)
        .await
        .map_err(|e| McpError::internal_error(format!("Domeneshop API error: {e}"), None))?
        .into_inner();
    Ok(domains)
}
