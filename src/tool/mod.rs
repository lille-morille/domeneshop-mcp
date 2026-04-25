pub mod create_dns_record;
pub mod delete_dns_record;
mod dns;
pub mod get_domain;
pub mod list_domains;
pub mod update_dns_record;

use rmcp::{
    ErrorData as McpError, RoleServer,
    handler::server::wrapper::Parameters,
    service::RequestContext,
    tool, tool_router,
};

use crate::{auth, server::DomeneshopServer};

#[tool_router(vis = "pub")]
impl DomeneshopServer {
    #[tool(
        description = "List all domains on the authenticated Domeneshop account as a JSON array \
            of `{id, domain}` pairs. Use this for discovery, then call `get_domain` for full \
            details on a specific domain."
    )]
    pub async fn list_domains(
        &self,
        ctx: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let client = auth::api_client_for(&ctx)?;
        list_domains::handle(&client).await
    }

    #[tool(
        description = "Fetch a single domain by id. Returns the same shape as one entry from \
            `list_domains`."
    )]
    pub async fn get_domain(
        &self,
        Parameters(params): Parameters<get_domain::Params>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let client = auth::api_client_for(&ctx)?;
        get_domain::handle(&client, params).await
    }

    #[tool(
        description = "Create a new DNS record on a Domeneshop-managed domain. \
            Supports A, AAAA, CNAME, MX, SRV, TLSA, and TXT."
    )]
    pub async fn create_dns_record(
        &self,
        Parameters(params): Parameters<create_dns_record::Params>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let client = auth::api_client_for(&ctx)?;
        create_dns_record::handle(&client, params).await
    }

    #[tool(
        description = "Replace an existing DNS record. All fields of the record are overwritten, \
            so you must supply the full record, not a diff."
    )]
    pub async fn update_dns_record(
        &self,
        Parameters(params): Parameters<update_dns_record::Params>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let client = auth::api_client_for(&ctx)?;
        update_dns_record::handle(&client, params).await
    }

    #[tool(description = "Delete a DNS record by its id.")]
    pub async fn delete_dns_record(
        &self,
        Parameters(params): Parameters<delete_dns_record::Params>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let client = auth::api_client_for(&ctx)?;
        delete_dns_record::handle(&client, params).await
    }
}
