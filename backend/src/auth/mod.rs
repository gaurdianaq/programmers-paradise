use actix_web::{get, HttpResponse, Responder};

//TODO, turn this into an actual auth route that does something
#[get("/auth")]
async fn auth_route() -> impl Responder {
    HttpResponse::Ok().body("Hello auth!")
}