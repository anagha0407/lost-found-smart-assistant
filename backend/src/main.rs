use actix_cors::Cors;
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(health)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}