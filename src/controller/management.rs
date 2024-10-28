
use actix_web::{get, HttpResponse, post, Responder, HttpRequest};
use serde_json::Value;
use crate::{errors, rsp};
use crate::rsp::success_rsp;
// const api_prefix: &str = "mgt";

// #[post("/mgt/login")]
pub(crate) async fn mgt_login(req_body: String) -> impl Responder {
  let mut payload_res: serde_json::Result<Value> = serde_json::from_str(req_body.as_str());
  match payload_res {
    Ok(payload) => {
      let username = payload.get("username").unwrap().as_str().unwrap();
      let password = payload.get("password").unwrap().as_str().unwrap();
      let mut rsp = success_rsp();
      if !(username == "neonrabbit" && password == "neonrabbit") {
        rsp = rsp::fail_rsp(errors::WRONG_USER.to_string());
      }
      rsp::http_success_rsp(&rsp)
    }
    Err(_) => {
      let rsp = rsp::fail_rsp(errors::HTTP_REQ_PARAMS.to_string());
      rsp::http_badreq_fail_rso(&rsp)
    }
  }
  // HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&rsp).unwrap())
}

// pub async fn register_service(req: HttpRequest, req_body: String) -> impl Responder {
//   todo!()
// }




