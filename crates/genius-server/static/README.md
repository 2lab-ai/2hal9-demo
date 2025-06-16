# Static Web UI

This directory contains the static web interface for the Genius Game Server.

## Files

- `index.html` - Main dashboard and API documentation
- Game demos are served from `/demo` directory

## Features

- Server status monitoring
- Game library browser
- API documentation with examples
- Quick start guide
- Links to interactive demos

## Development

To serve static files, the server needs to be configured to serve this directory at the root path.

Add to `genius-server/src/main.rs`:

```rust
use warp::fs;

// Serve static files
let static_files = warp::fs::dir("static");

// Combine with other routes
let routes = api_routes
    .or(ws_route)
    .or(static_files);
```