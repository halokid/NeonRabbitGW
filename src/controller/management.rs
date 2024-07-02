
use actix_web::{get, HttpResponse, post, Responder};
use actix_web::http::header::ContentType;
use crate::pkg::rsp::SuccessRsp;

// const api_prefix: &str = "mgt";

#[post("/mgt/login")]
pub(crate) async fn mgt_login() -> impl Responder {
  let rsp = SuccessRsp();
  HttpResponse::Ok().content_type(ContentType::json())
    .body(serde_json::to_string(&rsp).unwrap())
}


