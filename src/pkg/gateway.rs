use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse, post};
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use qstring::QString;
use crate::config::CONFIG;
use crate::service::client::Client;
use tokio::time::{sleep, Duration};
use crate::registry::registry::{RegApaptee, Registry};
use crate::service;

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

// unsafe impl Sync for RefCell {}

// #[derive(Debug)]
pub struct AppState {
  pub todo_db: Arc<Mutex<Vec<String>>>,
  pub clients: Arc<RwLock<HashMap<String, RefCell<Client>>>>,
}

impl AppState {
  pub fn init() -> AppState {
    AppState {
      todo_db: Arc::new(Mutex::new(Vec::new())),
      clients: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

pub struct Gateway {}

impl Gateway {
  pub fn new() -> Self {
    Gateway {}
  }

  pub async fn run(&self) -> std::io::Result<()> {
    log::info!("-->>> Gateway run");
    let app_state = AppState::init();
    let app_state_data = web::Data::new(app_state);
    let app_state_data_cl = app_state_data.clone();

    tokio::task::spawn(async move {
      update_clients(app_state_data_cl).await;
    });

    // tokio::task::spawn(
    //   futures::future::lazy(move |_| update_clients(app_state_data_cl) )
    // );

    let serv = HttpServer::new(move || {
      // App::new().service(gw_version);
      App::new()
        .app_data(app_state_data.clone())
        .wrap(Logger::default())
        .service(ping)
        .service(gw_version)
        .service(unify)
      // .service(unify_test)
    }).bind((CONFIG["gw_addr"], CONFIG["gw_port"].parse().unwrap()))?.run();
    serv.await
  }
}

fn get_keys(app_state_data: Data<AppState>) -> Vec<String> {
  let clients = Arc::clone(&app_state_data.clients);
  // log::debug!("-->>> Check update clients nodes loop start: {:?}", clients);
  let clients_rw = clients.read().unwrap();
  // log::debug!("Clients Arc clients_rw -->>> {:?}", clients_rw);
  let services = clients_rw.keys();
  // log::debug!("get_keys services -->>> {:?}", services);
  let mut keys = Vec::new();
  for k in services {
    keys.push(k.to_string());
  }
  drop(clients_rw);
  keys
}

async fn update_clients(app_state_data: Data<AppState>) {
  log::debug!("===>>> Check update clients nodes!!! <<<===");
  loop {
    sleep(Duration::from_secs(5)).await;
    let services = get_keys(app_state_data.clone());
    // log::debug!("update_clients services -->>> {:?}", services);
    // drop(clients_rw);
    let registry = Registry::new();
    for service in services {
      // get real time clients nodes
      // let service_nodes = registry.client.get_service("neon_broker").await;
      let service_nodes = registry.client.get_service(service.as_str()).await;
      let nodes = service_nodes.unwrap();
      // log::debug!("{} update_client nodes -->>> {:?}", service, nodes);
      // compare to exist clients nodes, see update or not
      update_client(service, nodes, app_state_data.clone());
    }
  }
}

fn update_client(client_key: String, nodes: Vec<String>, app_state_data: Data<AppState>) {
  // log::debug!("Start update_client app_state_data -->>> {:?}", app_state_data.clients);
  let clients = Arc::clone(&app_state_data.clients);
  let mut clients_rw = clients.write().unwrap();
  let client = clients_rw.get(client_key.as_str()).unwrap();
  client.borrow_mut().nodes = Arc::new(RwLock::new(nodes));
  drop(clients_rw);
  // log::debug!("update_client new {} app_state_data -->>> {:?}", client_key, app_state_data.clients);
}

#[post("/{service}/{method}")]
async fn unify(req: HttpRequest, req_body: String, data: web::Data<AppState>) -> impl Responder {
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

#[get("/ping")]
async fn ping() -> impl Responder {
  format!("ping!")
}

#[get("/version")]
async fn gw_version() -> impl Responder {
  format!("Gateway V1.0")
}


