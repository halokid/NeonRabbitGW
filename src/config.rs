use lazy_static::*;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc};
use tokio::sync::RwLock as TRwLock;

lazy_static! {
  pub static ref CONFIG: HashMap<&'static str, &'static str> = {
    let mut config = HashMap::new();
    let cmd_args: Vec<_> = env::args().collect();
    let mut run_env = "";
    if cmd_args.len() > 1 {
      run_env = &cmd_args[1];
    } else {
      run_env = "test"
    }

    config.insert("dapr_service_port", "4500"); // dapr, single
    // config.insert("gw_addr", "127.0.0.1");
    config.insert("gw_addr", "0.0.0.0");
    config.insert("gw_port", "8080");
    config.insert("log_level", "debug");
    // config.insert("log_level", "info");
    config.insert("registry_adaptee", "consul");
    // config.insert("consul_host", "127.0.0.1");
    config.insert("consul_host", "192.168.0.149");
    config.insert("consul_port", "8500");

    // Client selector
    config.insert("selector_type", "round_robin");

    match run_env {
      "dev" => {
      },
      "test" => {
      },
      "prd" => {
        config.insert("log_level", "info");
      },
      _ => {
      },
    }

    println!("Runtime Environment ----- {}: {:?}", run_env, config);
    config
  };
}

lazy_static! {
  // pub static ref  RUN_MODEL: String =  Arc::new(RwLock::new("single".to_string()));
  pub static ref  RUN_MODEL: Arc<TRwLock<String>> =  Arc::new(TRwLock::new(String::from("single")));
}
