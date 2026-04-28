## Domeneshop MCP

This repo is a http based MCP server for interacting with [Domeneshop's](https://domene.shop/) public API. This service forwards your own API keys to list, create, update or delete domain pointers.

The server does not support the full API, but instead focuses only on basic domain pointer CRUD.

## Getting started

You have two options:

### 1. Use my hosted MCP server on `domeneshop-mcp.morille.no`.

This is practical if you don't want to run/compile it locally, but will add some latency since all requests are proxied through this server. (But compared to LLM speeds it won't affect you that much)

Simply connect to that server and add the following header

`Authorization: Basic <token>:<secret>`

To obtain your token id and secret, go to the [API keys](https://domene.shop/admin?view=api) page on Domeneshop.

Here is the command if you wish to add it for Claude Code (globally):
`claude mcp add --transport http domeneshop https://domeneshop-mcp.morille.no --header "Authorization: Basic <token>:<secret>" --scope user`

### 2. Run it yourself

This server can be run using `cargo run` like a normal Rust project :).

You can change how it binds to a port using the `BIND` env-variable. The default value for this is `127.0.0.1:3000`. You might want to change that when running it, since port `3000` is commonly used for development projects.

Here is the command if you wish to add the local server to Claude Code (globally):
`claude mcp add --transport http domeneshop http://localhost:3000 --header "Authorization: Basic <token>:<secret>" --scope user`
