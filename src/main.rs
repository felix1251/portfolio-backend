use actix_web::{get, App, HttpServer};

//home path and serves as health checker
#[get("/")]
async fn index() -> String {
    "this is a health check".to_string()
}

//main actix web server to run
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
