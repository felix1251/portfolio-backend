use actix_json_response::JsonResponse;
use actix_web::{get, http::StatusCode, middleware::Logger, App, HttpServer, Result};
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct Welcome {
    msg: String,
}

//home path and serves as health checker
#[get("/")]
async fn index() -> Result<JsonResponse<Welcome>> {
    let welcome_obj = Welcome {
        msg: "Welcome to my portfolio API".to_string(),
    };
    Ok(JsonResponse::from(welcome_obj).with_status_code(StatusCode::OK))
}

//main actix web server to run
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
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
