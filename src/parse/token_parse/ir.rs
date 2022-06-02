#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Plurality {
  Singular,
  Plural,
  Unknown
}

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
  pub plurality: Plurality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRPerson {
  pub token_index: usize,
  pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IR {
  CoordinatingConjunction(IRCoordinatingConjunction),
  Delimiter(IRDelimiter),
  Action(IRAction),
  Entity(IREntity),
  Person(IRPerson)
}