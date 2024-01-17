use actix_web::{web, App, HttpResponse, HttpServer};

async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}