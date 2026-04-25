use axum::http::request::Parts;
use reqwest::header::{AUTHORIZATION, HeaderValue};
use rmcp::{ErrorData as McpError, RoleServer, service::RequestContext};

use crate::client::{self, ApiClient};

/// Pull the `Authorization` header off the HTTP request that carried this MCP
/// call, and build an authenticated Domeneshop API client from it.
///
/// The MCP server itself holds no Domeneshop credentials — every user's client
/// must send `Authorization: Basic <base64(token_name:secret)>` on each request.
pub fn api_client_for(ctx: &RequestContext<RoleServer>) -> Result<ApiClient, McpError> {
    let parts = ctx.extensions.get::<Parts>().ok_or_else(|| {
        McpError::internal_error(
            "request is missing HTTP parts; is the server configured for streamable-http?",
            None,
        )
    })?;
    let auth = authorization_header(parts)?;
    client::build(auth)
}

fn authorization_header(parts: &Parts) -> Result<HeaderValue, McpError> {
    let value = parts.headers.get(AUTHORIZATION).ok_or_else(|| {
        McpError::invalid_request(
            "Missing Authorization header. Configure your MCP client with HTTP Basic auth using \
             your Domeneshop API token name as username and its secret as password \
             (https://www.domeneshop.no/admin?view=api).",
            None,
        )
    })?;
    let s = value.to_str().map_err(|_| {
        McpError::invalid_request("Authorization header contains non-ASCII bytes", None)
    })?;
    if !s.starts_with("Basic ") {
        return Err(McpError::invalid_request(
            "Authorization must be HTTP Basic: `Basic <base64(token_name:secret)>`.",
            None,
        ));
    }
    Ok(value.clone())
}
