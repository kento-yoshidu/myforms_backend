use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    password: String,
}

#[post("/form3")]
pub async fn form3(form: web::Json<FormData>) -> impl Responder {
    HttpResponse::Ok().json({
        serde_json::json!({
            "message": "パスワードを設定しました！",
            "password": form.password,
        })
    })
}
