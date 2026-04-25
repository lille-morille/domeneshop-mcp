use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::{
        ListResourceTemplatesResult, ListResourcesResult, PaginatedRequestParams,
        ReadResourceRequestParams, ReadResourceResult, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool_handler,
};

use crate::resource;

#[derive(Clone)]
pub struct DomeneshopServer {
    pub tool_router: ToolRouter<Self>,
}

impl DomeneshopServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for DomeneshopServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler]
impl ServerHandler for DomeneshopServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Domeneshop MCP server. Manage DNS records and read domains/DNS on a \
                 Domeneshop account. Auth: configure your MCP client with HTTP Basic auth \
                 (username = API token name, password = API token secret — generate these at \
                 https://www.domeneshop.no/admin?view=api). The server forwards your credentials \
                 to api.domeneshop.no on every request and does not store them."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            ..Default::default()
        }
    }

    async fn list_resources(
        &self,
        _req: Option<PaginatedRequestParams>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        resource::list(&ctx).await
    }

    async fn read_resource(
        &self,
        req: ReadResourceRequestParams,
        ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        resource::read(req, &ctx).await
    }

    async fn list_resource_templates(
        &self,
        _req: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            resource_templates: resource::templates(),
            next_cursor: None,
            meta: None,
        })
    }
}
