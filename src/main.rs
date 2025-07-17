use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(Files::new("/assets", "./assets/").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
