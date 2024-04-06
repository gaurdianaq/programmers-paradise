use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod auth;
mod config;

use auth::auth_route;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let parsed_config = config::parse_config();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(auth_route)
    })
    .bind((parsed_config.addr, parsed_config.port))?
    .run()
    .await
}