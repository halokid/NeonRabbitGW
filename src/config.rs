use lazy_static::*;
use std::env;
use std::collections::HashMap;

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

    config.insert("gw_addr", "127.0.0.1");
    config.insert("gw_port", "8080");
    config.insert("log_level", "debug");
    config.insert("registry_adaptee", "consul");
    config.insert("consul_host", "127.0.0.1");
    config.insert("consul_port", "8500");

    // Client selector
    config.insert("selector_type", "round_robin");

    match run_env {
      "dev" => {
        // config.insert("log_level", "info");
        // config.insert("registry_adaptee", "consul");
        // config.insert("consul_host", "127.0.0.1");
        // config.insert("consul_port", "8500");
      },
      "test" => {
        // config.insert("log_level", "info");
        // config.insert("registry_adaptee", "consul");
        // config.insert("consul_host", "127.0.0.1");
        // config.insert("consul_port", "8500");
      },
      "prd" => {
        config.insert("log_level", "info");
      },
      _ => {
        // config.insert("log_level", "info");
        // config.insert("registry_adaptee", "consul");
        // config.insert("consul_host", "127.0.0.1");
        // config.insert("consul_port", "8500");
      },
    }

    println!("Runtime Environment ----- {}: {:?}", run_env, config);
    config
  };
}


