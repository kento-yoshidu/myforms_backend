mod form;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use lambda_web::{is_running_on_lambda, run_actix_on_lambda};
use std::error::Error;
use form::{
    form1::form1,
    form2::form2,
    form3::form3,
    form4::form4,
    form5::form5,
};

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
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
        .service(form1)
        .service(form2)
        .service(form3)
        .service(form4)
        .service(form5);

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
