use actix_web::{HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::vo::rsp::Rsp;

pub fn success_rsp() -> Rsp {
  Rsp {
    code: 0,
    message: "success".to_string(),
    data: "".to_string()
  }
}

pub fn success_rsp_data(data: String) -> Rsp {
  Rsp {
    code: 0,
    message: "success".to_string(),
    data: data,
  }
}

pub fn fail_rsp(mut message: String) -> Rsp {
  if message == "".to_string() {
    message = "Failt".to_string();
  }
  Rsp {
    code: 1,
    message: message,
    data: "".to_string()
  }
}

pub fn http_success_rsp(rsp: &Rsp) -> HttpResponse {
  HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(rsp).unwrap())
}

pub fn http_badreq_fail_rso(rsp: &Rsp) -> HttpResponse {
  HttpResponse::BadRequest().content_type(ContentType::json()).body(serde_json::to_string(rsp).unwrap())
}




