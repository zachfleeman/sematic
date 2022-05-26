#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate derive_more;

pub mod config;
pub mod routes;
pub mod sentence;
pub mod state;
pub mod services;
pub mod semantic_role_labeling;
pub mod nlp;
pub mod sema;
pub mod wordnet;
pub mod process_sentences;
pub mod parse;

use futures::lock::Mutex;
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};

use actix_web::{web, App, HttpServer};
use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

use routes::{index, text, srl, text2, text_to_json};
use state::State;

use crate::nlp::nlp_rule::NLPRule;
use crate::wordnet::wordnet_verbs::WordnetVerbs;
use crate::wordnet::wordnet_nouns::WordnetNouns;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  color_backtrace::install();
  let config = config::server_config();

  let link_parser = Arc::new(Mutex::new(LinkParser::new(LinkParserOptions {})));

  // Init NLP Rule
  NLPRule::init();

  // Init Wordnet collections
  WordnetVerbs::init();
  WordnetNouns::init();

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
      .service(index)
      .service(text)
      .service(srl)
      .service(text2)
      .service(text_to_json)
  })
  .bind(format!("0.0.0.0:{}", config.tcp_port))?
  .run()
  .await
}
