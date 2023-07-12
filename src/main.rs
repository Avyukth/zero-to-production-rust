use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main()->std::io::Result<()>{
    run().await
}

