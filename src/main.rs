use env_logger::Env;
use std::env;
use std::sync::{Arc, RwLock};
use futures_util::TryFutureExt;
use tokio::io::AsyncReadExt;
use NeonRabbitGW::pkg::gateway::Gateway;

mod config;
use config::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or(CONFIG["log_level"])).init();

  let run_model: Arc<RwLock<String>> = Arc::clone(&RUN_MODEL);
  let run_model_r = run_model.read().unwrap();
  println!("run_model_r -->>> {}", run_model_r);
  drop(run_model_r);

  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let mut run_model_rw = run_model.write().unwrap();
    *run_model_rw = args.get(1).unwrap().to_string();
    println!("run_model_rw -->>> {}", run_model_rw);
    drop(run_model_rw);

    let run_model_update: Arc<RwLock<String>> = Arc::clone(&RUN_MODEL);
    println!("run_model_update -->>> {}", run_model_update.read().unwrap());
  }

  let gw = Gateway::new();
  gw.run().await
}
