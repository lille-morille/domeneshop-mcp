use rmcp::{
    ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool_handler,
};

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
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
