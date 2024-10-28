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
use actix_web::{dev::Service as _};
use crate::middleware::heartbeat::Heartbeat;
use crate::middleware::middleware::MiddleWare;

unsafe impl Send for AppState {}

unsafe impl Sync for AppState {}

// unsafe impl Sync for RefCell {}

// #[derive(Debug)]
pub struct AppState {
  // pub todo_db: Arc<Mutex<Vec<String>>>,
  // TODO: why here use `ReCell` for `Client`? cuz we need a mut ref in `gateway.unify` controller
  // TODO: we have a process need update the service client ref there, this scenaior need `refcell`
  // TODO: we ca use `borrow_mut()` to a `refcell` variable to update it
  pub clients: Arc<RwLock<HashMap<String, RefCell<Client>>>>,
}

impl AppState {
  pub fn init() -> AppState {
    AppState {
      // todo_db: Arc::new(Mutex::new(Vec::new())),
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
        .wrap(MiddleWare)
        // .wrap(Heartbeat)
        .app_data(app_state_data.clone())
        .wrap(Logger::default())
        .route("/ping", web::get().to(controller::gateway::ping))
        .route("/health", web::get().to(controller::gateway::health))
        .route("/version", web::get().to(controller::gateway::gw_version))
        .route("/mgt/login", web::post().to(controller::management::mgt_login))
        .route("/{service}/{method}", web::post().to(controller::gateway::unify))
        .route("/{service}/{method}", web::get().to(controller::gateway::unify))
        // .service(controller::gateway::ping)
        // .service(controller::gateway::gw_version)
        // .service(controller::management::mgt_login)
        // .service(controller::gateway::unify)
        .default_service(web::route().to(controller::gateway::not_found))
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

fn update_client(client_key: String, nodes: Vec<String>, app_state_data: Data<AppState>) {
  // log::debug!("Start update_client app_state_data -->>> {:?}", app_state_data.clients);
  let clients = Arc::clone(&app_state_data.clients);
  let mut clients_rw = clients.write().unwrap();
  let client = clients_rw.get(client_key.as_str()).unwrap();
  client.borrow_mut().nodes = Arc::new(RwLock::new(nodes));
  drop(clients_rw);
  // log::debug!("update_client new {} app_state_data -->>> {:?}", client_key, app_state_data.clients);
}

