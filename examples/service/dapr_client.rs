
// todo: load proto generate lib
pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

use std::error;
use std::thread;
use std::time;
use hello_world::{HelloReply, HelloRequest};
use prost::Message;
use std::str;


type DaprClient = dapr::Client<dapr::client::TonicClient>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  println!("-->>> Dapr client");
  // let addr = format!("https://127.0.0.1:3600");
  // let address = format!("https://127.0.0.1:19527");
  let address = format!("https://127.0.0.1:3500");

  let mut client = DaprClient::connect(address).await?;

    let request = HelloRequest {
        name: "Test Halokid".to_string(),
    };
    let data = request.encode_to_vec();
    let data = prost_types::Any {
        type_url: "".to_string(),
        value: data,
    };

    let response = client
        .invoke_service("neon_broker", "ping",
                        Some(data))
        .await
        .unwrap();

    if let Some(any) = &response.data {
        let data = &any.value;
        let resp = HelloReply::decode(&data[..]).unwrap();
        println!("Message: {:#?}", &resp.name);
    };

    println!("Response: {:#?}", response);
    println!("Response value: {:#?}", str::from_utf8(&response.data.unwrap().value).unwrap());

    Ok(())
}



