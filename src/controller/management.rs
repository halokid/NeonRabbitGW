
use actix_web::{get, HttpResponse, post, Responder, HttpRequest};
use actix_web::http::header::ContentType;
use crate::pkg::rsp::success_rsp;

// const api_prefix: &str = "mgt";

// #[post("/mgt/login")]
pub(crate) async fn mgt_login() -> impl Responder {
  let rsp = success_rsp();
  HttpResponse::Ok().content_type(ContentType::json())
    .body(serde_json::to_string(&rsp).unwrap())
}

// pub async fn register_service(req: HttpRequest, req_body: String) -> impl Responder {
//   todo!()
// }




