
use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::{Field, Multipart};
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;
use actix_web::rt::Runtime;
use actix_web::web::Json;
use exitfailure::ExitFailure;
use futures::{StreamExt, TryStreamExt};
use log::{debug, error, info, Level, log_enabled};
use rust_bert::pipelines::common::ModelType;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::mem::drop;
use std::path::PathBuf;
use std::sync::Once;
use std::thread;
use tch::Device;


async fn save_file(mut field: Field) -> Result<String, std::io::Error> {
    let mut upload_dir = PathBuf::from(std::env::current_dir().unwrap());
    upload_dir.push("upload");

    if !upload_dir.exists() {
        match fs::create_dir(&upload_dir) {
            Ok(_) => println!("Created directory: {:?}", upload_dir),
            Err(e) => panic!("Failed to create directory {:?}: {}", upload_dir, e),
        }
    }
    let mut file_name = None;
    let content_disposition = field.content_disposition();
    if let Some(name) = content_disposition.get_filename() {
        let upload_dir_string = upload_dir.to_string_lossy().to_string();

        file_name = Some(format!("{}/{}", upload_dir_string, name));
    }
    let file_path = file_name.unwrap();

    println!("{}", file_path);

    let mut file = std::fs::File::create(file_path.clone())?;

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file.write_all(&data)?;
    }

    Ok(file_path)
}


#[derive(Serialize)]
struct FileResult {
    message: String,
}

#[post("/api/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    let mut results = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        match save_file(field).await {
            Ok(file_path) => {
                println!("result: {}", result);
                results.push(FileResult {
                    message: file_path.to_string(),
                });
            }
            Err(err) => {
                results.push(FileResult {
                    message: format!("Error: {}", err),
                });
            }
        }
    }

    if results.is_empty() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::Ok().json(Json(results.into_iter().next().unwrap()))
    }
}

#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Server started successfully");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            // .wrap(cors) // Add the CORS middleware to the app
            .service(upload)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}