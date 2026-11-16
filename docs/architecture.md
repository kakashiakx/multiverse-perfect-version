# Project Architecture

This repository contains a static HTML dashboard, a Node.js/Express web server and REST API, and a separate Rust/Actix Web API.

## Frontend

- `index.html` is the browser entry point served at `/` by the Node.js app and by Nginx in the Docker Compose setup.
- The page is branded **OMEGA-PRIME** and presents the multiverse-themed repository dashboard: telemetry cards, performance and test charts, timeline visualization, and an architecture overview.
- Tailwind CSS, Chart.js, and Plotly are loaded from public CDNs. Chart and dashboard values are illustrative content defined in the HTML; the page does not fetch live telemetry from either API.

## Services and routes

### Node.js / Express (`api/`)

- `GET /` serves the repository's `index.html`.
- `GET /api/health` returns the Node API health status.
- `GET /api/hello?name=traveler` returns a greeting; the optional name is trimmed and limited to 80 characters.

### Rust / Actix Web (`src/main.rs`)

- `GET /health` returns the Rust service health status.
- `GET /hello` returns a greeting.
- The service listens on port `8080` by default; `HOST` and `PORT` can override its bind address and port.

## Deployment

`docker-compose.yml` defines the Node API, Rust API, and Nginx services. Nginx serves the frontend and proxies API requests to the Node service. The services expose ports `80`, `3000`, and `8080` respectively, and include health checks.

## Key files

- `index.html` — dashboard UI and chart setup.
- `api/server.js` — static page handler and Express API routes.
- `src/main.rs` — Rust API routes and server setup.
- `nginx/nginx.conf` — reverse-proxy and frontend serving configuration.
- `docker-compose.yml` — local multi-service deployment.
- `README.md` — setup and run instructions.
