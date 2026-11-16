use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "multiverse-perfect-version-rust",
        "message": "A perfect version from the multiverse."
    }))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello from Rust in the multiverse!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust API running on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
