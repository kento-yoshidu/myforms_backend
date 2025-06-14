mod form;

use actix_web::{http, get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use lambda_web::{is_running_on_lambda, run_actix_on_lambda};
use std::error::Error;
use form::{form1::form1};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = ||
        App::new()
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["POST", "GET"])
                .allowed_headers(vec![http::header::CONTENT_TYPE])
                .max_age(3600),
        )
        .service(form1);

    if is_running_on_lambda() {
        run_actix_on_lambda(app).await
    } else {
        HttpServer::new(app)
            .bind(("0.0.0.0", 8000))?
            .run()
            .await
            .map_err(|e| e.into())
    }
}
