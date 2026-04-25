use rmcp::ErrorData as McpError;

use crate::client::ApiClient;

pub async fn handle(client: &ApiClient) -> Result<String, McpError> {
    client.fetch_text("/domains").await
}
