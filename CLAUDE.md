# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build              # Build (regenerates API client if openapi.json changed)
cargo run                # Start the MCP server (defaults to 127.0.0.1:3000)
cargo check              # Fast type-check
cargo clippy             # Lint — pedantic + nursery + many restriction lints are denied/warned
cargo fmt                # Format
BIND=0.0.0.0:8080 cargo run   # Override listen address
```

There is no test suite. The crate has only a single binary target.

## Runtime shape

The binary is an MCP server speaking over **streamable HTTP** (`rmcp` 0.16). It exposes:

- `GET /healthz` — liveness probe
- `/mcp` — the `StreamableHttpService` mount

`Config::from_env` reads only `BIND`. There are no other env-driven knobs.

## Auth model — credentials are forwarded, never stored

The server itself holds no Domeneshop credentials. Every inbound MCP request must carry an HTTP `Authorization: Basic <base64(token_name:secret)>` header. `auth::api_client_for` pulls it from the request's `axum::http::Parts` (made available because the streamable-http transport stuffs them into `RequestContext::extensions`) and builds a per-request `ApiClient`. If you add a new tool/resource handler, take `RequestContext` and call `auth::api_client_for(&ctx)` — do not try to construct a client any other way.

## Two HTTP clients in `ApiClient`

`client::ApiClient` bundles two clients with the same auth headers:

- `generated` — typed progenitor client used by **tools** (create/update/delete writes).
- `http` — raw `reqwest::Client` used by **resources** (reads).

The split exists because the real Domeneshop API returns some integer fields (`MX.priority`, `SRV.priority`/`weight`/`port`) as JSON strings, which the typed schema rejects. Resources use `ApiClient::fetch_text` to forward the upstream JSON verbatim. When adding read paths, prefer `fetch_text`; when adding writes, use the typed client.

## Build-time API client generation

`build.rs` reads `openapi.json` and runs `progenitor` to emit `$OUT_DIR/codegen.rs`, which `src/api.rs` includes. Two transforms happen before generation:

1. **`inject_operation_ids`** — many paths in the upstream spec lack `operationId`. We synthesize `<method>_<path_segments>` so progenitor produces stable method names.
2. **`inline_subproperty_refs`** — `typify` (used by progenitor) rejects `$ref`s that point into a schema sub-property (e.g. `…/Invoice/properties/status`). We resolve those pointers in-place before handing the spec to progenitor.

If you regenerate after editing `openapi.json` and the build fails inside `typify`, the most likely cause is a new sub-property `$ref` or a missing `operationId` — extend the build script rather than mutating `openapi.json` by hand.

`src/api.rs` blanket-allows lints for the generated module; do not lint-fix into the generated file.

## MCP surface

All capabilities are exposed as **tools** (no MCP resources). Tools are model-controlled, which matters because most clients (Claude Code included) don't auto-attach resources to context — the model can't read them on its own.

`src/tool/`:
- `list_domains`, `get_domain` — reads. Use `ApiClient::fetch_text` and forward upstream JSON verbatim.
- `create_dns_record`, `update_dns_record`, `delete_dns_record` — writes via the typed `generated` client. They share `tool::dns::build_dns_record`, which maps a flat `Params` shape (with `record_type` + record-specific optional fields) onto the typed `DnsRecord` enum variants and validates required fields per record type. Update is a full replace, not a patch.

## Lint posture

`Cargo.toml` denies `clippy::all` + `clippy::pedantic` and warns on `nursery`/`cargo` plus many restriction lints (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `print_stdout`, `print_stderr`, etc.). Use `tracing` for logging — `println!`/`eprintln!` will trip lints. Prefer `?` and explicit `McpError::*` constructors over `unwrap`/`expect`.
