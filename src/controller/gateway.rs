use std::cell::RefCell;
use std::sync::Arc;
use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use qstring::QString;
use crate::pkg::gateway::AppState;
use tokio::time::{sleep, Duration};
use crate::service::client::Client;

// #[get("/ping")]
pub async fn ping() -> impl Responder {
  format!("ping!")
}

pub async fn health() -> impl Responder {
  format!("health")
}

pub async fn not_found() -> impl Responder {
  // format!("Error 404")
  HttpResponse::NotFound().body(format!("Error 404"))
}

// #[get("/version")]
pub async fn gw_version() -> impl Responder {
  format!("Gateway V1.0")
}

// #[post("/{service}/{method}")]
pub async fn unify(req: HttpRequest, req_body: String, data: web::Data<AppState>) -> impl Responder {
  log::debug!("\n\n================= <<--- unify call start -->>> ==================");

  // just for test pause request 5 seconds to verify concurrency
  // log::debug!("AppState clients -->>> {:?}", data.clients);
  // for  parameter style like  `url?concurr-test=yes`
  let qs = QString::from(req.query_string());
  let concurr_test = qs.get("concurr-test");
  match concurr_test {
    None => {}
    Some(_) => {
      sleep(Duration::from_secs(5)).await;
    }
  }

  let http_method = req.method().as_str();
  // parse url
  let service: String = req.match_info().query("service").parse().unwrap();
  let method: String = req.match_info().query("method").parse().unwrap();
  log::debug!("-->>> service: {}, method: {}", service, method);

  // let mut clients = data.clients.lock().unwrap();
  let mut clients = data.clients.read().unwrap();
  log::debug!("All Clients -->>> {:?}", clients);

  let client_svc_res = clients.get(service.as_str());
  let mut payload = serde_json::from_str(req_body.as_str());
  // TODO: initial the `serde_json::Value`
  let mut body: serde_json::Value = serde_json::Value::Null;
  match payload {
    Ok(..) => {
      // log::debug!("-->>> unify request body is not null");
      body = payload.unwrap();
      log::debug!("unify request body -->>> {:?}", body);
    }
    Err(err) => {
      // log::error!("-->>> unify get request body error: {:?}", err);
      log::debug!("-->>> unify get request no body payload!");
    }
  }
  let mut rsp = "".to_string();

  match client_svc_res {
    Some(client_svc) => {
      log::debug!("-->>> Gateway exist client");
      let mut client_run = client_svc.borrow_mut();
      let svc_rsp = client_run.invoke(service, method, http_method.to_string(), body).await;
      // let svc_rsp =  Client::invoke(service, method, body).await;
      log::debug!("exist Client svc_rsp -->>> {:?}", svc_rsp);
      rsp = svc_rsp.unwrap();
    }
    None => {
      drop(clients);    // TODO: relaese the `read` lock of the Arc for below `write`
      log::debug!("-->>> Gateway new client");
      let mut client = Client::new();
      let svc_rsp_res = client.invoke(service.clone(), method, http_method.to_string(), body).await;
      match svc_rsp_res {
        Ok(svc_rsp) => {
          // clients.insert(service.clone(), client);
          {
            let clients = Arc::clone(&data.clients);
            let mut clients_rw = clients.write().unwrap();
            clients_rw.insert(service, RefCell::new(client));
            drop(clients_rw);
          }
          /*
          tokio::task::spawn(async move {
          // tokio::task::spawn(async move || {
            let mut k = cs.write().unwrap();
            // k.insert(service.clone(), client);
            k.insert(service.clone(), RefCell::new(client) );
          });
           */
          log::debug!("new Client svc_rsp -->>> {:?}", svc_rsp);
          rsp = svc_rsp;
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

