use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{env, fs::File};

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Config {
  pub logging_directive: String,
  pub tcp_port: u16,
  pub graceful_shutdown_timeout_sec: u64,
  pub max_payload_size_bytes: usize,
  pub database_connection_pool_size: u32,
  pub database_connection_timeout_sec: u64,
  pub database_url: String,
  pub use_jwt_auth: bool,
  pub jwt_secret: String,
  pub data_path: String,
  pub duckling_url: String,
}

fn init() -> Config {
  if let Ok(path) = env::var("CONFIG_PATH") {
    let config = File::open(&path).unwrap_or_else(|error| {
      panic!(
        "Could not open configuration file specified by env var CONFIG_PATH={}: {}",
        path, error
      )
    });
    ron::de::from_reader::<File, Config>(config)
  } else {
    let config = std::env::var("CONFIG")
      .unwrap_or_else(|_| panic!("Neither CONFIG_PATH nor CONFIG env vars were set."));

    ron::de::from_str::<Config>(&config)
  }
  .unwrap_or_else(|error| panic!("Could not parse configuration file: {}", error))
}

/// Returns the global configuration for the server.
pub fn server_config() -> &'static Config {
  CONFIG.get_or_init(init)
}