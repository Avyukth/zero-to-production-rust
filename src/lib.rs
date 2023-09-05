use actix_web::dev::{Payload, Server};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub trait FromRequest: Sized {
    type Error = Into<actix_web::Error>;
    async fn from_request(req: &HttpRequest, Payload: &mut Payload) -> Result<Self, Self::Error>;
}
