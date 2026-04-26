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

pub fn build(mut authorization: HeaderValue) -> Result<ApiClient, McpError> {
    authorization.set_sensitive(true);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization);

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

    /// Resolve a domain name to its Domeneshop numeric id.
    ///
    /// The upstream `?domain=` filter is a substring match, so we filter the
    /// returned list client-side for an exact match on the `domain` field.
    pub async fn domain_id_by_name(&self, name: &str) -> Result<i64, McpError> {
        let url = format!("{BASE_URL}/domains");
        let resp = self
            .http
            .get(&url)
            .query(&[("domain", name)])
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
        let domains: Vec<serde_json::Value> = serde_json::from_str(&body)
            .map_err(|e| McpError::internal_error(format!("parsing /domains: {e}"), None))?;
        let id = domains
            .iter()
            .find(|d| d.get("domain").and_then(serde_json::Value::as_str) == Some(name))
            .and_then(|d| d.get("id").and_then(serde_json::Value::as_i64));
        id.ok_or_else(|| McpError::invalid_params(format!("domain {name:?} not found"), None))
    }
}
