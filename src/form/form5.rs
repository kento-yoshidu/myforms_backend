use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
    email: String,
}

#[post("/form5")]
pub async fn form5(form: web::Json<FormData>) -> impl Responder {
    HttpResponse::Ok().json({
        serde_json::json!({
            "message": "ユーザーを登録しました！",
            "username": form.username,
            "email": form.email,
        })
    })
}
