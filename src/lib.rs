use actix_web::{HttpResponse, HttpServer, App, web};


async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub async fn run()->std::io::Result<()> {
        HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

