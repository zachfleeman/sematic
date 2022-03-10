extern crate serde_derive;
extern crate serde_json;

use futures::lock::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

pub mod sentence;

use sentence::Sentence;

#[get("/{id}")]
async fn index(id: web::Path<i32>) -> impl Responder {
  let id = id.into_inner();
  format!("Hello {}!", id)
}

#[post("/text")]
async fn text(payload: String, link_parser: web::Data<Arc<Mutex<LinkParser>>>) -> impl Responder {
  let lp = link_parser
    .lock()
    .await;

  let lp_sentence = lp
    .parse_sentence(payload.clone())
    .expect("Hope this works");

  let sentence = Sentence::from_lp_sentence(lp_sentence.clone());

  // let a = serde_json::to_value(sentence).expect("Hope this works 2");
  let a = serde_json::to_value(lp_sentence).expect("Hope this works 2");

  // let json = serde_json::to_string_pretty(&a).expect("JSON");

  HttpResponse::Ok().json(a)
  // HttpResponse::Ok().body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let link_parser = Arc::new(Mutex::new(LinkParser::new(LinkParserOptions {})));

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(link_parser.clone()))
      .service(index)
      .service(text)
  })
  .bind("0.0.0.0:8089")?
  .run()
  .await
}
