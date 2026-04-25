pub const DEFAULT_BASE_URL: &str = "https://api.domeneshop.no/v0";
pub const DEFAULT_BIND: &str = "127.0.0.1:3000";

pub struct Config {
    pub base_url: String,
    pub bind: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("DOMENESHOP_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_BASE_URL.into()),
            bind: std::env::var("DOMENESHOP_MCP_BIND").unwrap_or_else(|_| DEFAULT_BIND.into()),
        }
    }
}
