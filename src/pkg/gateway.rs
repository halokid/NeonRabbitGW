use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::config::CONFIG;
use crate::service::client::Client;
use tokio::time::{sleep, Duration};
use crate::controller;
use crate::registry::registry::{Registry};

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
    // TODO: all web service runtime can share use the `app_state`
    let app_state_data = web::Data::new(app_state);
    let app_state_data_cl = app_state_data.clone();

    // tokio::task::spawn(async move {
    //   update_clients(app_state_data_cl).await;
    // });

    // tokio::task::spawn(
    //   futures::future::lazy(move |_| update_clients(app_state_data_cl) )
    // );

    let serv = HttpServer::new(move || {
      // Define allowed origins
      let allowed_origins = "http://localhost:3000";
      let cors = Cors::default() // Allow all origins
        .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Specify allowed HTTP methods
        .allowed_origin(allowed_origins)
        .allowed_headers(vec![ // Add allowed headers
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
        .supports_credentials()
        .max_age(3600); // Cache preflight responses for 1 hour

      // App::new().service(gw_version);
      App::new()
        .wrap(cors)
        .app_data(app_state_data.clone())
        .wrap(Logger::default())
        .service(controller::gateway::ping)
        .service(controller::gateway::gw_version)
        .service(controller::management::mgt_login)
        .service(controller::gateway::unify)
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

/*
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
 */


fn update_client(client_key: String, nodes: Vec<String>, app_state_data: Data<AppState>) {
  // log::debug!("Start update_client app_state_data -->>> {:?}", app_state_data.clients);
  let clients = Arc::clone(&app_state_data.clients);
  let mut clients_rw = clients.write().unwrap();
  let client = clients_rw.get(client_key.as_str()).unwrap();
  client.borrow_mut().nodes = Arc::new(RwLock::new(nodes));
  drop(clients_rw);
  // log::debug!("update_client new {} app_state_data -->>> {:?}", client_key, app_state_data.clients);
}

/*
#[get("/ping")]
async fn ping() -> impl Responder {
  format!("ping!")
}

#[get("/version")]
async fn gw_version() -> impl Responder {
  format!("Gateway V1.0")
}
 */


