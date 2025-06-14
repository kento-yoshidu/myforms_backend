use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
}

#[post("/form1")]
pub async fn form1(form: web::Json<FormData>) -> impl Responder {
    let uppercased_name = form.name.to_uppercase();

    HttpResponse::Ok().json({
        serde_json::json!({
            "message": "あなたの名前を大文字に変換しました！",
            "name": uppercased_name,
        })
    })
}
