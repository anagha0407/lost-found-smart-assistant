use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use std::sync::Mutex;

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(
        serde_json::json!({
            "status": "Backend is working"
        })
    )
}


#[derive(Deserialize, Serialize, Clone)]
struct Report {
    item_name: String,
    description: String,
    location: String,
    date: String,
    report_type: String,
}

struct AppState {
    reports: Mutex<Vec<Report>>,
}

#[post("/api/reports")]
async fn create_report(
    report: web::Json<Report>,
    data: web::Data<AppState>,
) -> impl Responder {

    println!("Received report:");
    println!("{}", report.item_name);

    let mut reports = data.reports.lock().unwrap();
    reports.push(report.into_inner());
    println!("Total reports: {}", reports.len());

    HttpResponse::Ok().json(
        serde_json::json!({
            "message": "Report received successfully"
        })
    )
}

#[get("/api/reports")]
async fn get_reports(
    data: web::Data<AppState>,
) -> impl Responder {

    let reports = data.reports.lock().unwrap();

    HttpResponse::Ok().json(reports.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server running on http://127.0.0.1:3001");

    let app_state = web::Data::new(
        AppState {
            reports: Mutex::new(Vec::new()),
        }
    );

    HttpServer::new(move || {

    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

    App::new()
        .app_data(app_state.clone())
        .wrap(cors)
        .service(health)
        .service(create_report)
        .service(get_reports)
    })
    .bind(("127.0.0.1",3001))?
    .run()
    .await
}