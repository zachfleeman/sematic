#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRDelimiter {
  pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRCoordinatingConjunction {
  pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRAction {
  pub token_index: usize,
  pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IREntity {
  pub token_index: usize,
  pub text: String,
  pub plural: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IR {
  CoordinatingConjunction(IRCoordinatingConjunction),
  Delimiter(IRDelimiter),
  Action(IRAction),
  Entity(IREntity),
}