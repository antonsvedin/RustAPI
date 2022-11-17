use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("TODO: Start Page")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("TODO: List Users from DB")
}

#[get("/user/{username}")]
async fn get_user(_username: String) -> impl Responder {
    HttpResponse::Ok().body("TODO: Get single user by username from DB")
}

#[post("/add_user")]
async fn add_user() -> impl Responder {
    HttpResponse::Ok().body("TODO: Add user to DB")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(get_users)
        .service(get_user)
        .service(add_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
