use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::path::PathBuf;

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

#[get("/")]
async fn homepage() -> HttpResponse {
    match std::fs::read(PathBuf::from("index.html")) {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(contents),
        Err(_) => HttpResponse::NotFound().body("index.html not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Rust web app and API running at http://{host}:{port}");
    HttpServer::new(|| App::new().service(homepage).service(health).service(hello))
        .bind((host, port))?
        .run()
        .await
}
