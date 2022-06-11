// https://www.abisource.com/projects/link-grammar/dict/section-P.html

// P is used to link forms of the verb "be" to various words that can be its complements: 
// prepositions, adjectives, and passive and progressive participles.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PComplements {
  Preposition,
  Adjective,
  PresentParticiple,
  PassiveParticiple,
  // ProgressiveParticiple, // Is this needed, or used?
  Unknown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P {
  pub complement: Option<PComplements>,
}