use std::future::Future;
use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::dev::ServiceResponse;

#[get("/samplemw")]
pub async fn samplemw() -> impl Responder {
  format!("sample mw!")
}

// pub fn samplecmw_call() -> ServiceResponse {}
