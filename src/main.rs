use env_logger::Env;
use NeonRabbitGW::pkg::gateway::Gateway;

mod config;
use config::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default()
    .default_filter_or(CONFIG["log_level"]))
    .init();
  let gw = Gateway::new();
  gw.run().await
}


