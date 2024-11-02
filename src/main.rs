use env_logger::Env;
use std::env;
use std::sync::{Arc};
use futures_util::TryFutureExt;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock as TRwLock;
use NeonRabbitGW::pkg::gateway::Gateway;

mod config;
use config::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or(CONFIG["log_level"])).init();

  let run_model: Arc<TRwLock<String>> = Arc::clone(&RUN_MODEL);
  let run_model_r = run_model.read().await;
  println!("run_model_r -->>> {}", run_model_r);
  drop(run_model_r);

  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let mut run_model_rw = run_model.write().await;
    *run_model_rw = args.get(1).unwrap().to_string();
    println!("run_model_rw -->>> {}", run_model_rw);
    drop(run_model_rw);

    let run_model: Arc<TRwLock<String>> = Arc::clone(&RUN_MODEL);
    let run_model_r = run_model.read().await;
    println!("run_model_update -->>> {:?}", run_model_r);
  }

  let gw = Gateway::new();
  gw.run().await
}
