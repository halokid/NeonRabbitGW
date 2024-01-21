use std::fmt::Error;
use async_trait::async_trait;
use crate::{config, CustomErr};

#[async_trait]
pub trait RegClient {
  async fn get_service(&self, service: &str) -> Result<Vec<String>, CustomErr>;
}

// pub trait Selector {
//   fn select(&mut self) -> String;
//   fn update_serv_nodes(&mut self, serv_new_nodes: Vec<String>) -> CakeResult<bool>;
// }

pub enum RegApaptee {
  Consul,
  Nacos,
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}

pub struct Registry {
  adaptee: RegApaptee,
  pub client: Box<dyn RegClient + Send + Sync>,
}

impl Registry {
  pub fn new() -> Self {
    let consul_host = config::CONFIG["consul_host"];
    let consul_port = config::CONFIG["consul_port"].parse().unwrap();
    let adapter = crate::registry::consul::Consul::new(consul_host, consul_port);
    let client = Box::new(adapter);

    match config::CONFIG["registry_adaptee"] {
      "nacos" => {
        Registry {
          adaptee: RegApaptee::Nacos,
          client,
        }
      }
      _ => {
        Registry {
          adaptee: RegApaptee::Consul,
          client,
        }
      }
    }
  }
}

/*
#[cfg(test)]
mod tests {
  use crate::CustomErr;
  use crate::registry::registry::{RegApaptee, RegClient, Registry};

  #[test]
  fn test_get_service() {
    assert_eq!(4, 4);
    let mut registry = Registry::new();
    // registry.set_client();
    // let nodes = registry.client.get_service("neon_broker").await;
    // println!("nodes -->>> {:?}", nodes);
  }
}
 */


