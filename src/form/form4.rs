use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
}

#[post("/form4")]
pub async fn form4(form: web::Json<FormData>) -> impl Responder {
    HttpResponse::Ok().json({
        serde_json::json!({
            "message": "メールアドレスを登録しました！",
            "email": form.email,
        })
    })
}
