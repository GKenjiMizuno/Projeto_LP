use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyData {
    message: String,
}

async fn get_data() -> impl Responder {
    let data = MyData {
        message: String::from("Hello from Rust!"),
    };
    web::Json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())  // Adiciona middleware CORS
            .route("/api/data", web::get().to(get_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
