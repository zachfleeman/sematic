extern crate serde_derive;
extern crate serde_json;

use anyhow::{self, Result};
use derive_more::Display;
use futures::lock::Mutex;
use futures::TryFutureExt;

use serde_json::json;

use std::sync::Arc;

use crate::process_sentences::process::process_parts;
use crate::services::allennlp_service::{get_semantic_role_labels, SRLResponse};

use actix_web::{get, post, web, Error, HttpResponse, Responder};
use link_parser_rust_bindings::{LinkParser, LinkParserError};

use crate::nlp::sentence_parts::SentenceParts;

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

impl From<LinkParserError> for SemaAPiError {
  fn from(err: LinkParserError) -> Self {
    SemaAPiError { err: err.into() }
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
  pub parts: Option<bool>,
  pub repair: Option<bool>,
}

#[post("/text-to-json")]
async fn text_to_json(
  payload: web::Json<TextToJSONRequestObject>,
  link_parser: web::Data<Arc<Mutex<LinkParser>>>,
) -> Result<impl Responder, Error> {
  let lp = link_parser
    .lock()
    .await;

  let mut all_parts = vec![];

  let repair = payload.repair.unwrap_or(false);

  for sentence in payload
    .sentences
    .iter()
  {
    let mut parts = SentenceParts::from_text(sentence, repair).map_err(SemaAPiError::from)?;

    if let Some(links) = lp
      .parse_sentence(&parts.corrected_sentence)
      .map_err(SemaAPiError::from)?
    {
      parts.links = links;
    }

    all_parts.push(parts);
  }

  let sema_sentences = process_parts(all_parts.clone())
    .await
    .map_err(SemaAPiError::from)?;

  let resp = if payload
    .parts
    .unwrap_or(false)
  {
    json!({
      "sema_sentences": sema_sentences,
      "parts": &all_parts,
    })
  } else {
    json!({
      "sema_sentences": sema_sentences,
    })
  };

  Ok(HttpResponse::Ok().json(resp))
}