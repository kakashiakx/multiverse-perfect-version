use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::json;

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({
        "status": "ok",
        "service": "multiverse-perfect-version-rust",
        "message": "A perfect version from the multiverse."
    }))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(json!({
        "message": "Hello from Rust in the multiverse!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);

    println!("Rust API running at http://127.0.0.1:{port}");
    HttpServer::new(|| App::new().service(health).service(hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
