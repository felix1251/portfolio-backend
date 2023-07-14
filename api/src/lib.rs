use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Result};
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct Welcome {
    mesage: String,
}

//home path and serves as health checker
#[get("/")]
async fn welcome() -> Result<HttpResponse> {
    let welcome_obj = Welcome {
        mesage: "welcome to my portfolio API".to_string(),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(welcome_obj))
}

//main actix web server to run
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(welcome)
            .wrap(Logger::new("%a %r status: %s %Dms"))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
