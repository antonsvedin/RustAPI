use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use RustAPI::calculate;

#[get("/")]
async fn index() -> impl Responder {
    let x: i32 = 289;
    let response: String = calculate(x);
    HttpResponse::Ok().body(format!("{} x {} is {}", x, x, response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
