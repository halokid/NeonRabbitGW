use std::fmt::Error;

pub struct Client;

impl Client {

  pub async fn invoke(service: &str) -> Result<String, Error> {
    // get service address

    // invoke service
  }
}