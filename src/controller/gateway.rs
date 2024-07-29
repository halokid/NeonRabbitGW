use std::cell::RefCell;
use std::sync::Arc;
use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use qstring::QString;
use crate::pkg::gateway::AppState;
use tokio::time::{sleep, Duration};
use crate::service::client::Client;

#[get("/ping")]
pub async fn ping() -> impl Responder {
  format!("ping!")
}

pub async fn not_found() -> impl Responder {
  // format!("Error 404")
  HttpResponse::NotFound().body(format!("Error 404"))
}

#[get("/version")]
pub async fn gw_version() -> impl Responder {
  format!("Gateway V1.0")
}

#[post("/{service}/{method}")]
pub async fn unify(req: HttpRequest, req_body: String, data: web::Data<AppState>) -> impl Responder {
  log::debug!("\n\n================= <<--- unify call start -->>> ==================");
  // parse url
  let service: String = req.match_info().query("service").parse().unwrap();
  let method: String = req.match_info().query("method").parse().unwrap();
  log::debug!("-->>> service: {}, method: {}", service, method);

  // let mut clients = data.clients.lock().unwrap();
  let mut clients = data.clients.read().unwrap();
  log::debug!("All Clients -->>> {:?}", clients);

  // log::debug!("AppState clients -->>> {:?}", data.clients);
  let qs = QString::from(req.query_string());
  let test = qs.get("test");
  match test {
    None => {}
    Some(_) => {
      sleep(Duration::from_secs(5)).await;
    }
  }

  let client_wp = clients.get(service.as_str());
  // let client_wp = clients.get(service.as_str());
  // let mut body: serde_json::Value = serde_json::from_str(req_body.as_str()).unwrap();
  let mut body_check = serde_json::from_str(req_body.as_str());
  // TODO: initial the `serde_json::Value`
  let mut body: serde_json::Value = serde_json::Value::Null;
  match body_check {
    Ok(..) => {
      // log::debug!("-->>> unify request body is not null");
      body = body_check.unwrap();
      log::debug!("unify request body -->>> {:?}", body);
    }
    Err(err) => {
      log::error!("-->>> unify get request body error: {:?}", err);
    }
  }
  let mut rsp = "".to_string();

  match client_wp {
    Some(clientx) => {
      log::debug!("-->>> Gateway exist client");
      let mut client_run = clientx.borrow_mut();
      let svc_rsp = client_run.invoke(service, method, body).await;
      // let svc_rsp =  Client::invoke(service, method, body).await;
      log::debug!("exist Client svc_rsp -->>> {:?}", svc_rsp);
      rsp = svc_rsp.unwrap();
    }
    None => {
      drop(clients);    // TODO: relaese the `read` lock of the Arc for below `write`
      log::debug!("-->>> Gateway new client");
      let mut client = Client::new();
      let svc_rsp = client.invoke(service.clone(), method, body).await;
      match svc_rsp {
        Ok(rspx) => {
          // clients.insert(service.clone(), client);
          {
            let cs = Arc::clone(&data.clients);
            let mut k = cs.write().unwrap();
            k.insert(service, RefCell::new(client));
            drop(k);
          }
          /*
          tokio::task::spawn(async move {
          // tokio::task::spawn(async move || {
            let mut k = cs.write().unwrap();
            // k.insert(service.clone(), client);
            k.insert(service.clone(), RefCell::new(client) );
          });
           */
          log::debug!("new Client svc_rsp -->>> {:?}", rspx);
          rsp = rspx;
        }
        Err(err) => {
          rsp = err.0;
        }
      }
    }
  }

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(rsp)
}

