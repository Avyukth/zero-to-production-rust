use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(index))
        .route("/{name}", web::get().to(index))
        .route("/health_check", web::get().to(health_check))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
