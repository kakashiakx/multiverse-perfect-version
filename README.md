# multiverse-perfect-version

*A perfect version from the multiverse.*

A small starter project with an HTML frontend, a Node.js/Express REST API, and a Rust/Actix Web API.

## Run the web app and REST API (Node.js)

Requirements: Node.js 18 or newer and npm.

```bash
cd api
npm install
npm start
```

Open <http://localhost:3000>. The page calls the API on the same origin. Try `/api/health` and `/api/hello?name=traveler` directly. Set `PORT` to use a different port.

## Run the Rust API

Requirements: Rust and Cargo (install via [rustup](https://rustup.rs/)). From the repository root:

```bash
cargo run
```

The Rust API listens on <http://127.0.0.1:8080>; endpoints are `/health` and `/hello`. Set `PORT` to change its port.

## Project layout

- `index.html` — frontend served by the Node.js app.
- `api/` — Express REST API and its npm manifest.
- `Cargo.toml`, `src/main.rs` — separate Rust/Actix Web API.
- `docs/` — project documentation.

The Node and Rust APIs are independent services; running one does not start the other.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before submitting changes. For security reports, follow [SECURITY.md](SECURITY.md).

## License

This project is distributed under the [MIT License](LICENSE).
