use anyhow::Result;
use crate::config::server_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DucklingParseResponse {
  pub body: String,

  pub start: usize,

  pub end: usize,

  pub dim: String, // replace with enum

  pub latent: bool,

  pub value: DucklingValueOption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DucklingValueOption {
  Value { grain: String, value: String, values: Option<Vec<DucklingValueOption>> },
  Interval { to: NoTypeValue, from: NoTypeValue, values: Option<Vec<DucklingValueOption>> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoTypeValue {
  pub value: String,

  pub grain: String,
}

pub async fn duckling_parse_sentence(sentence: &str) -> Result<Vec<DucklingParseResponse>> {
  let client = reqwest::Client::new();
  let params = [("locale", "en_US"), ("text", sentence)];
  let resp = client
    .post(server_config().duckling_url.as_str())
    .form(&params)
    .send()
    .await?;

  let duckling_resp = resp
    .json::<Vec<DucklingParseResponse>>()
    .await?;

  Ok(duckling_resp)
}
