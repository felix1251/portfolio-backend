use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer};
use env_logger::Env;

//home path and serves as health checker
#[get("/")]
async fn index() -> String {
    "this is a health check".to_string()
}

//main actix web server to run
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .service(index)
            .wrap(Logger::new("%a %r status: %s %Dms"))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
