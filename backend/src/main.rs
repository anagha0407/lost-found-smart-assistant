use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "Backend is working"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:3001");

    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}