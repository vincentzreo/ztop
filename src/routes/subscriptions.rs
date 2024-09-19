use actix_web::{web, HttpResponse};

use super::FormData;

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
