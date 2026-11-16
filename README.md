# multiverse-perfect-version

*A perfect version from the multiverse.*

A small starter project with an HTML frontend, a Node.js/Express REST API, and a Rust/Actix Web API.

## Run locally

### Web app + REST API (Node.js)

Requirements: Node.js 18 or newer and npm.

```bash
cd api
npm install
npm start
```

Open http://localhost:3000. The page calls `/api/health` and `/api/hello?name=traveler`.

### Rust API

Requirements: Rust and Cargo.

```bash
cargo run
```

The Rust API listens on http://127.0.0.1:8080. Endpoints: `/health` and `/hello`.

## Run everything with Docker Compose

```bash
docker compose up --build
```

Then:
- Frontend + Node API: http://localhost:3000
- Rust API: http://localhost:8080/health

## Project layout

- `index.html` — frontend served by the Node.js app.
- `api/` — Express REST API and npm manifest.
- `Cargo.toml`, `src/main.rs` — Rust/Actix Web API.
- `docs/` — project documentation.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before submitting changes. For security reports, follow [SECURITY.md](SECURITY.md).

## License

This project is distributed under the [MIT License](LICENSE).
