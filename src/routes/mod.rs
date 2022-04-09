extern crate serde_derive;
extern crate serde_json;

use derive_more::Display;
use futures::lock::Mutex;
use futures::TryFutureExt;
use link_parser_rust_bindings::lp::word::LPWord;
use serde_json::json;
use std::sync::Arc;

use crate::semantic_role_labeling::play_with_srl;
use crate::services::allennlp_service::{get_semantic_role_labels, SRLResponse};

use crate::sentence::entities::Entity;
use crate::sentence::{EntityModifer, Sentence, SentenceTypes};
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use link_parser_rust_bindings::{lp::sentence::LPSentence, pos::POS, LinkParser};

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

#[get("/{id}")]
async fn index(id: web::Path<i32>) -> Result<impl Responder, Error> {
  let id = id.into_inner();
  Ok(format!("Hello {}!", id))
}

#[post("/srl")]
async fn srl(payload: String) -> Result<impl Responder, Error> {
  let srl_resp: SRLResponse = get_semantic_role_labels(&payload)
    .map_err(SemaAPiError::from)
    .await?;

  Ok(HttpResponse::Ok().json(srl_resp))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPayload {
  sentence: Sentence,
  lp_sentence: LPSentence,
}

#[post("/text")]
async fn text(
  payload: String,
  link_parser: web::Data<Arc<Mutex<LinkParser>>>,
) -> Result<impl Responder, Error> {
  let lp = link_parser
    .lock()
    .await;

  // let srl_resp = get_srl(&payload)
  //   .map_err(SemaAPiError::from)
  //   .await?;

  let lp_sentence = lp
    .parse_sentence(payload.clone())
    .expect("Hope this works");

  // Need to determine all the actions (verbs) in the sentence.
  let mut sentence = Sentence::new(SentenceTypes::Imperative);

  // Get entities (nouns) from the sentence
  for lp_word in lp_sentence
    .clone()
    .words
    .into_iter()
  {
    if let Some(pos) = lp_word.pos {
      match pos {
        POS::Noun => {
          let modifiers = get_entity_modifiers(&lp_word, lp_sentence.clone());
          let entity = Entity {
            entity_type: lp_word.word.clone(),
            handle: format!("${}", lp_word.word.clone()),
            modifiers: vec![],
          };

          sentence
            .entities
            .push(entity);
        }
        _ => {}
      };
    }
  }

  let payload = TextPayload {
    sentence,
    lp_sentence,
  };

  let lp_json = serde_json::to_value(payload)?;

  Ok(HttpResponse::Ok().json(lp_json))
}

pub fn get_entity_modifiers(lp_word: &LPWord, lp_sentence: LPSentence) -> Vec<EntityModifer> {
  // for disjunct in lp_word.disjuncts.iter() {
  //   match
  // }

  vec![]
}

#[post("/text2")]
async fn text2(
  payload: String, // link_parser: web::Data<Arc<Mutex<LinkParser>>>,
) -> Result<impl Responder, Error> {
  play_with_srl(&payload)
    .map_err(SemaAPiError::from)
    .await?;

  // Ok(HttpResponse::Ok().json(srl_resp))
  Ok(HttpResponse::Ok())
}
