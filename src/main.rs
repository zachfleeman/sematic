#[macro_use]
extern crate serde_derive;
extern crate derive_more;
extern crate serde_json;

pub mod config;
pub mod nlp;
pub mod parse;
pub mod process_sentences;
pub mod routes;
pub mod sema;
pub mod sentence;
pub mod services;
pub mod state;
pub mod wordnet;
pub mod verify;

use futures::lock::Mutex;
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};

use actix_web::{web, App, HttpServer};
use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

use routes::{health, srl, text_to_json};
use state::State;

use crate::nlp::human_names::HumanNames;
use crate::nlp::nlp_rule::NLPRule;
use crate::wordnet::wordnet_nouns::WordnetNouns;
use crate::wordnet::wordnet_verbs::WordnetVerbs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  color_backtrace::install();
  let config = config::server_config();

  let link_parser_ops = LinkParserOptions {
    verbosity: 0,
    ..LinkParserOptions::default()
  };
  let link_parser = Arc::new(Mutex::new(LinkParser::new(link_parser_ops)));

  // Init NLP Rule
  NLPRule::init();

  // Init Wordnet collections
  WordnetVerbs::init();
  WordnetNouns::init();

  // Init Human Names
  HumanNames::init();

  dbg!(&config);

  let pool = PgPoolOptions::new()
    .max_connections(config.database_connection_pool_size)
    .connect_timeout(Duration::new(config.database_connection_timeout_sec, 0))
    .connect(&config.database_url)
    .await
    .expect("Error connecting to database");

  let state = State::new(pool.clone());

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(link_parser.clone()))
      .app_data(web::Data::new(state.clone()))
      .service(health)
      .service(srl)
      .service(text_to_json)
  })
  .bind(format!("0.0.0.0:{}", config.tcp_port))?
  .run()
  .await
}
