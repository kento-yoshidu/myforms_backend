use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda};
use std::error::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = || App::new().service(hello);

    if is_running_on_lambda() {
        run_actix_on_lambda(app).await
    } else {
        HttpServer::new(app)
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
            .map_err(|e| e.into())  // ← ここで型変換
    }
}
