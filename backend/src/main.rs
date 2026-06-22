use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LostItem {
    item_name: String,
    description: String,
    location: String,
}

#[derive(Deserialize)]
struct FoundItem {
    item_name: String,
    description: String,
    location: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "Backend is working"
    }))
}

#[post("/api/lost-items")]
async fn create_lost_item(item: web::Json<LostItem>) -> impl Responder {
    println!(
        "Lost Item: {} | {} | {}",
        item.item_name,
        item.description,
        item.location
    );

    HttpResponse::Ok().json(ApiResponse {
        message: "Lost item received".to_string(),
    })
}

#[post("/api/found-items")]
async fn create_found_item(item: web::Json<FoundItem>) -> impl Responder {
    println!(
        "Found Item: {} | {} | {}",
        item.item_name,
        item.description,
        item.location
    );

    HttpResponse::Ok().json(ApiResponse {
        message: "Found item received".to_string(),
    })
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
            .service(create_lost_item)
            .service(create_found_item)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}