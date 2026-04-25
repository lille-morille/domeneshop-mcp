use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use rmcp::ErrorData as McpError;

use crate::{api, config::BASE_URL};

/// Authenticated Domeneshop bundle built per inbound MCP request.
///
/// * `generated` — typed progenitor client used by tools (create / update / delete).
/// * `http` — raw `reqwest` client used by resources so we can forward
///   Domeneshop's JSON verbatim. This sidesteps upstream quirks where the real
///   API returns integers as strings (MX `priority`, SRV
///   `priority`/`weight`/`port`), which the typed schema rejects.
#[derive(Clone)]
pub struct ApiClient {
    pub generated: api::Client,
    pub http: reqwest::Client,
}

pub fn build(authorization: HeaderValue) -> Result<ApiClient, McpError> {
    let mut auth = authorization;
    auth.set_sensitive(true);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, auth);

    let http = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(concat!("domeneshop-mcp/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| {
            McpError::internal_error(format!("building upstream HTTP client: {e}"), None)
        })?;

    let generated = api::Client::new_with_client(BASE_URL, http.clone());
    Ok(ApiClient { generated, http })
}

impl ApiClient {
    /// GET a path on the Domeneshop API and return the response body as a
    /// UTF-8 string. Non-2xx responses produce an MCP error that includes the
    /// status and upstream body.
    pub async fn fetch_text(&self, path: &str) -> Result<String, McpError> {
        let url = format!("{BASE_URL}{path}");
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| McpError::internal_error(format!("GET {url}: {e}"), None))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| McpError::internal_error(format!("reading {url}: {e}"), None))?;
        if !status.is_success() {
            return Err(McpError::internal_error(
                format!("Domeneshop API {status} for {url}: {body}"),
                None,
            ));
        }
        Ok(body)
    }
}
