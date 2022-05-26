use anyhow::Result;
use std::collections::HashMap;

use crate::sema::sema_sentence::SemaSentence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemaAIRequest {
  pub sentences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemaJSONAndSentence {
  pub json: SemaSentence,
  pub sentence: String,
}

type SemaAIResponse = HashMap<String, SemaJSONAndSentence>;

pub async fn get_ml_generated_sentence(raw_sentences: Vec<String>) -> Result<SemaAIResponse> {
  let client = reqwest::Client::new();
  let body = SemaAIRequest { sentences: raw_sentences };

  let resp = client.post("http://54.69.0.8:5050/text-to-json")
  .json::<SemaAIRequest>(&body)
  .send()
  .await?;

  let sema_resp = resp.json::<SemaAIResponse>().await?;

  Ok(sema_resp)
}