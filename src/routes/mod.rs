extern crate serde_derive;
extern crate serde_json;

use anyhow::Result;
use derive_more::Display;
use futures::lock::Mutex;
use futures::TryFutureExt;

use serde_json::json;

use std::sync::Arc;

use crate::process_sentences::process::process_parts;
use crate::services::allennlp_service::{get_semantic_role_labels, SRLResponse};

use actix_web::{get, post, web, Error, HttpResponse, Responder};
use link_parser_rust_bindings::{LinkParser};

use crate::nlp::{sentence_parts::SentenceParts,};

// This custom error is needed to convert between anyhow::Error and actix_web::Error
#[derive(Debug, Display)]
pub struct SemaAPiError {
  err: anyhow::Error,
}

impl actix_web::error::ResponseError for SemaAPiError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
      "error": self.err.to_string(),
    }))
  }
}

impl From<anyhow::Error> for SemaAPiError {
  fn from(err: anyhow::Error) -> Self {
    SemaAPiError { err }
  }
}

#[get("/health")]
async fn health() -> Result<impl Responder, Error> {
  Ok(HttpResponse::Ok())
}

#[post("/srl")]
async fn srl(payload: String) -> Result<impl Responder, Error> {
  let srl_resp: SRLResponse = get_semantic_role_labels(&payload)
    .map_err(SemaAPiError::from)
    .await?;

  Ok(HttpResponse::Ok().json(srl_resp))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToJSONRequestObject {
  pub sentences: Vec<String>,
}

#[post("/text-to-json")]
async fn text_to_json(
  payload: web::Json<TextToJSONRequestObject>,
  link_parser: web::Data<Arc<Mutex<LinkParser>>>,
) -> Result<impl Responder, Error> {
  let lp = link_parser
  .lock()
  .await;

  let parts = payload
    .sentences
    .iter()
    .filter_map(|sentence| {
      let links = lp
        .parse_sentence(&sentence)
        .expect("Hope this works");
      SentenceParts::from_text(sentence.clone(), links).ok()
    })
    .collect::<Vec<SentenceParts>>();

  let sema_sentences = process_parts(parts.clone())
    .await
    .map_err(SemaAPiError::from)?;

  Ok(HttpResponse::Ok().json(json!({
    "sema_sentences": sema_sentences,
    "parts": parts
  })))
}
