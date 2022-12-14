#[macro_use]
extern crate serde_derive;
extern crate derive_more;
extern crate serde_json;

pub mod config;
pub mod middleware;
pub mod nlp;
pub mod parse;
pub mod process_sentences;
pub mod routes;
pub mod sema;
pub mod sentence;
pub mod services;
pub mod state;
pub mod verify;
pub mod wordnet;

use futures::lock::Mutex;
use sema_api::config::{server_config, Config};
// use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
// use std::{sync::Arc, time::Duration};

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

use routes::{health, srl, text_to_json};
// use state::State;

use crate::middleware::auth::validator;
use crate::nlp::init_nlp_cells;
use crate::wordnet::init_wordnet_cells;
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::DecodingKey;

fn get_cors(config: &Config) -> Cors {
  let mut cors = Cors::default()
    .allowed_methods(vec!["GET", "POST", "PATCH", "OPTIONS"])
    .allow_any_header()
    .supports_credentials();

  for origin in config.allowed_origins.clone() {
    cors = cors.allowed_origin(&origin);
  }

  cors
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // openssl_probe::init_ssl_cert_env_vars();
  println!("current dir:{:?}", std::env::current_dir());
  color_backtrace::install();
  let config = server_config();

  let mut log_builder = env_logger::Builder::new();
  log_builder.parse_filters(&config.logging_directive);
  log_builder.init();

  let link_parser_ops = LinkParserOptions {
    verbosity: 0,
    ..LinkParserOptions::default()
  };
  let link_parser = Arc::new(Mutex::new(LinkParser::new(link_parser_ops)));

  init_nlp_cells(&config.data_path);
  init_wordnet_cells(&config.data_path);

  dbg!(&config);

  // let pool = PgPoolOptions::new()
  //   .max_connections(config.database_connection_pool_size)
  //   .connect_timeout(Duration::new(config.database_connection_timeout_sec, 0))
  //   .connect(&config.database_url)
  //   .await
  //   .expect("Error connecting to database");

  // let state = State::new(pool.clone());

  HttpServer::new(move || {
    let decoding_key = DecodingKey::from_secret(
      config
        .jwt_secret
        .as_bytes(),
    );

    App::new()
      .wrap(actix_web::middleware::Logger::new("%s for %U %a in %Ts"))
      .wrap(HttpAuthentication::bearer(validator))
      .app_data(web::Data::new(link_parser.clone()))
      // .app_data(web::Data::new(state.clone()))
      .wrap(get_cors(config))
      .app_data(decoding_key)
      .service(health)
      .service(srl)
      .service(text_to_json)
  })
  .bind(format!("0.0.0.0:{}", config.tcp_port))?
  .run()
  .await
}
