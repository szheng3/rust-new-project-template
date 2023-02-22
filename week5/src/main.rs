use actix_files ::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

use serde::Serialize;
use serde::Deserialize;
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api_health_handler)
            .service(

            Files::new("/", "./dist").index_file("index.html")
            ,
        )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}