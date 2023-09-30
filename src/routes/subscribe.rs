use actix_web::{web, HttpResponse};
use serde::Deserialize;

pub async fn subscribe(form: web::Form<SubscribeFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct SubscribeFormData {
    email: String,
    name: String,
}
