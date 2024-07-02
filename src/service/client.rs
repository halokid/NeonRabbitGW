use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use actix_web::cookie::time::Month::March;
use log::log;
use tonic::codegen::ok;
use crate::config::CONFIG;
use crate::CustomErr;
use crate::pkg::errors::RSP_ERR_NO_NODES;
use crate::registry::registry::{RegApaptee, Registry};
use crate::service::selector::rounbin::Roundbin;
use crate::service::selector::random::Random;
use crate::service::selector::selector::{Adaptee, Selector};

/*
#[derive(Debug)]
pub struct Client {
  selector: Box<dyn SelectorTr>,
  // selector: Selector,
}
 */

#[derive(Debug)]
pub struct Client {
  selector_type: Adaptee,
  selector: Box<dyn Selector>,
  pub nodes: Arc<RwLock<Vec<String>>>,
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
  pub fn new() -> Self {
    let selector_type = CONFIG["selector_type"];

    let mut selector_model = Adaptee::RoundRobin;
    let mut selector = Roundbin::new();
    let mut nodes: Vec<String> = Vec::new();

    match selector_type {
      "random" => {
        selector_model = Adaptee::Random;
        let selector = Random::new();
      }
      _ => {}
    }

    Client {
      selector_type: selector_model,
      selector: Box::new(selector),
      // nodes: Arc::new(RwLock::from(Vec::new())),
      nodes: Arc::new(RwLock::new(nodes)),
    }
  }

  fn set_selector(&self) -> Adaptee {
    let selector_type = CONFIG["selector_type"];
    match selector_type {
      "round_robin" => {
        Adaptee::RoundRobin
      }
      "random" => {
        Adaptee::Random
      }
      _ => {
        Adaptee::RoundRobin
      }
    }
  }

  pub async fn invoke(&mut self, service_name: String, method: String,
                      body: serde_json::Value) -> Result<String, CustomErr> {
    if CONFIG["model"] == "dapr" {
       return self._invoke_dapr(service_name, method, body).await;
    }

    // get nodes from client
    let client_nodes = Arc::clone(&self.nodes);   // client.nodes
    let client_nodes_r = client_nodes.read().unwrap();
    let mut nodes = Vec::new();
    if client_nodes_r.len() == 0 {
      drop(client_nodes_r);
      log::debug!("-->>> Client nodes is 0, add nodes");

      let registry = Registry::new();
      let service_nodes = registry.client.get_service(service_name.as_str()).await;
      log::debug!("service_nodes -->>> {:#?}", service_nodes);
      nodes = service_nodes.unwrap();

      if nodes.len() == 0 {
        log::warn!("{}", RSP_ERR_NO_NODES);
        return Err(CustomErr(RSP_ERR_NO_NODES.to_string()));
      }

      let mut client_nodes_w = client_nodes.write().unwrap();
      client_nodes_w.extend(nodes.clone());
    }

    // invoke service, specify `post`
    let http_client = reqwest::Client::new();
    // read cient nodes
    let client_nodes_r = client_nodes.read().unwrap();
    // let node = client_nodes_r.get(0).unwrap();
    let node = self.selector.select_node(client_nodes_r);
    let url = format!("http://{}/{}", node, method);
    // let url = format!("http://{}/{}", nodes[0], method);
    log::debug!("Client invoke url -->>> {:?}", url);
    let res = http_client.post(url)
      .json(&body)
      .send().await.unwrap().text().await;

    match res {
      Ok(res) => {
        Ok(res)
      }
      Err(err) => {
        log::error!("-->>> Client invoke err: {:?}", err);
        Err(CustomErr("-->>> Client invoke err, please check".to_string()))
      }
    }
    // Ok("Client invoke".to_string())
  }

  // invoke sevice use dapr way
  // TODO: why here need use GW to call dapr service? cuz we need drft the route call in Zipkin
  // TODO: if direct call dapr service here, will not record the route link `GW --> neon_schedule`
  // TODO: in zipkin, of course we need cost a little performance for this, but this way can be more
  // TODO: visibility and trace fro the micro service
  async fn _invoke_dapr(&mut self, service_name: String,  method: String, body: serde_json::Value)
    -> Result<String, CustomErr> {
    let http_client = reqwest::Client::new();
    let url = format!("http://{}:{}/{}", CONFIG["gw_addr"],
                      CONFIG["dapr_service_port"], method);
    log::debug!("Client _invoke_dapr url -->>> {:?}", url);
    let res = http_client.post(url)
      .header("dapr-app-id", service_name)
      .json(&body)
      .send().await.unwrap().text().await;

    match res {
      Ok(res) => {
        Ok(res)
      }
      Err(err) => {
        log::error!("-->>> Client _invoke_dapr err: {:?}", err);
        Err(CustomErr("-->>> Client _invoke_dapr err, please check".to_string()))
      }
    }
  }
}

/*
#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use env_logger::Env;
  use reqwest;
  use crate::service::client::Client;

  #[test]
  fn test_service_invoke() {
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // TODO: `rt.block_on` can support both `sync` and `async` fn
    rt.block_on(async {
      let client = reqwest::Client::new();
      // let mut map = HashMap::new();
      // map.insert("name", "halokid");
      let map = serde_json::json!({
        "name": "halokid",
      });
      let res = client.post("http://localhost:19527/ping")
        .json(&map)
        .send().await.unwrap().text().await;

      println!("res -->>> {:?}", res);

      // ---------------------------------------------------
      let mut client = Client::new();
      let svc_rsp = client.invoke("neon_broker".to_string(), "ping".to_string(), map).await;
      println!("svc_rsp -->>> {}", svc_rsp.unwrap());
    });
  }
}
 */

