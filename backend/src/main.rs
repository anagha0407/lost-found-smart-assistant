use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use actix_cors::Cors;

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(
        serde_json::json!({
            "status": "Backend is working"
        })
    )
}


#[derive(Deserialize)]
struct Report {
    item_name: String,
    description: String,
    location: String,
    date: String,
    report_type: String,
}


#[post("/api/reports")]
async fn create_report(
    report: web::Json<Report>
) -> impl Responder {

    println!("Received report:");
    println!("{}", report.item_name);

    HttpResponse::Ok().json(
        serde_json::json!({
            "message": "Report received successfully"
        })
    )
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
        .service(create_report)
    })
    .bind(("127.0.0.1",3001))?
    .run()
    .await
}