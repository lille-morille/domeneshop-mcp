use rmcp::{
    ErrorData as McpError,
    model::{RawResource, ResourceContents},
};

use crate::client::ApiClient;

pub const URI: &str = "domeneshop://domains";

pub fn descriptor() -> RawResource {
    let mut res = RawResource::new(URI, "domains");
    res.title = Some("All Domeneshop domains".into());
    res.description = Some(
        "JSON array of all domains on the authenticated account, with id, \
         registrar status, expiry date, and enabled services."
            .into(),
    );
    res.mime_type = Some("application/json".into());
    res
}

pub async fn read(client: &ApiClient) -> Result<Vec<ResourceContents>, McpError> {
    let body = client.fetch_text("/domains").await?;
    Ok(vec![ResourceContents::TextResourceContents {
        uri: URI.into(),
        mime_type: Some("application/json".into()),
        text: body,
        meta: None,
    }])
}
