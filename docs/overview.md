# Project Overview

`multiverse-perfect-version` is a multiverse-themed web dashboard backed by two small API services. The frontend is defined in `index.html`; its OMEGA-PRIME presentation includes telemetry cards, performance and test charts, and a timeline visualization.

## Frontend

The page is served at `/` by the Express app and through Nginx in the Docker Compose setup. It uses Tailwind CSS, Chart.js, and Plotly from public CDNs. The dashboard data and chart values are illustrative and are embedded in the page; they are not live metrics returned by the APIs.

## Services

- **Node.js / Express** (`api/`): serves `index.html`, provides `GET /api/health`, and provides `GET /api/hello?name=traveler`.
- **Rust / Actix Web** (`src/main.rs`): provides `GET /health` and `GET /hello`.
- **Nginx** (`nginx/`): reverse proxy and frontend entry point in the Docker Compose deployment.

## Run locally

For prerequisites and commands to run the frontend and APIs individually or with Docker Compose, see the [README](../README.md).

## Main files

- `index.html` — dashboard layout, styling, and chart initialization.
- `api/server.js` — Express frontend handler and API routes.
- `src/main.rs` — Rust API routes.
- `docker-compose.yml` — local multi-service configuration.
- `docs/architecture.md` — architecture and deployment details.
