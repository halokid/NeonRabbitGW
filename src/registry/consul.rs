use async_trait::async_trait;
use crate::CustomErr;
use crate::registry::registry::RegClient;
// use consul_rs_plus::Client;

unsafe impl Send for Consul {}
unsafe impl Sync for Consul {}

pub struct Consul {
  // client: Client,
}

impl Consul {
  pub fn new(host: &str, port: u16) -> Self {
    // let client = Client::new(host, port);
    Consul{
      // client,
    }
  }
}

#[async_trait]
impl RegClient for Consul {

  async fn get_service(&self, service_name: &str) -> Result<Vec<String>, CustomErr> {
    // let nodes = self.client.service_get(service_name.to_string()).await;
    // Ok(nodes)
    let vs = Vec::new();
    Ok(vs)
  }
}




