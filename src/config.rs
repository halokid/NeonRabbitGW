use lazy_static::*;
use std::env;
use std::collections::HashMap;

lazy_static! {
  pub static ref GLOBAL_CONFIG: HashMap<&'static str, &'static str> = {
    let mut config = HashMap::new();
    let cmd_args: Vec<_> = env::args().collect();
    let mut run_env = "";
    if cmd_args.len() > 1 {
      run_env = &cmd_args[1];
    } else {
      run_env = "test"
    }

    match run_env {
      "dev" => {
        config.insert("log_level", "info");
      },
      "test" => {
        config.insert("log_level", "info");
      },
      "prd" => {
        config.insert("log_level", "info");
      },
      _ => {},
    }

    println!("Runtime Environment ----- {}: {:?}", run_env, config);
    config
  };
}


