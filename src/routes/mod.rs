extern crate serde_derive;
extern crate serde_json;

use futures::lock::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, Error, HttpResponse, Responder};
use link_parser_rust_bindings::{LinkParser};

#[get("/{id}")]
async fn index(id: web::Path<i32>) -> Result<impl Responder, Error> {
  let id = id.into_inner();
  Ok(format!("Hello {}!", id))
}

#[post("/text")]
async fn text(payload: String, link_parser: web::Data<Arc<Mutex<LinkParser>>>) -> Result<impl Responder, Error> {
  let lp = link_parser
    .lock()
    .await;

  let lp_sentence = lp
    .parse_sentence(payload.clone())
    .expect("Hope this works");


  // Need to determine all the actions (verbs) in the sentence.


  let lp_json = serde_json::to_value(lp_sentence)?;

  Ok(HttpResponse::Ok().json(lp_json))
}