use super::{location::Directions, symbol::Symbol};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Args {
  V,
  ARG0(ArgMods),
  ARG1(ArgMods),
  ARG2(ArgMods),
  ARG3(ArgMods),
  ARG4(ArgMods),
  ARGM(ArgMMods),
  R(Box<Args>),
  Unknown,
}

// More deets at: https://verbs.colorado.edu/~mpalmer/projects/ace/PBguidelines.pdf
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgMods {
  None,
  // Proto-agent: 
  // - volitional involvement in the event or state
  // - causing an event or change of state in another participant
  // - movement relative to the position of another participant
  PAG,

  // Proto-patient:
  // - undergo change of state 
  // - are causally affected by another participant
  // - are stationary relative to movement of another participant
  PPT,
  PRD, // secondary predication
  GOL, // goal, recipient, beneficiary (target?) ("targeting" me)
  DIR, // direction
  MNR, // manner, instrument
  PRP, // purpose
  VSP, // Verbs specific
  TMP, // Temporal, but this may be a mistake in the propbank data
  COM, // Comitative
  LOC, // Location
  CAU, // Cause
}

impl ArgMods {
  pub fn from_str(s: &str) -> Self {
    // println!("s: {}", s);
    match s.to_uppercase().as_str() {
      "PAG" => ArgMods::PAG,
      "PPT" => ArgMods::PPT,
      "PRD" => ArgMods::PRD,
      "GOL" => ArgMods::GOL,
      "DIR" => ArgMods::DIR,
      "MNR" => ArgMods::MNR,
      "PRP" => ArgMods::PRP,
      "VSP" => ArgMods::VSP,
      "TMP" => ArgMods::TMP,
      "COM" => ArgMods::COM,
      "LOC" => ArgMods::LOC,
      "CAU" => ArgMods::CAU,
      _ => ArgMods::None,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArgMMods {
  COM, // comitative, indicate accompaniment (with another agent)
  LOC, // locative
  DIR, // directional
  GOL, // goal
  MNR, // manner
  TMP, // temporal
  EXT, // extent
  REC, // reciprocals
  PRD, // secondary predication
  PRP, // purpose
  CAU, // cause
  DIS, // discourse
  ADV, // adverb
  ADJ, // adjectival
  MOD, // modal
  NEG, // negation
  DSP, // direct speech
  LVB, // light verb
  CXN, // construction
  Unknown,
}

impl ArgMMods {
  pub fn from_str(s: &str) -> Self {
    match s.to_uppercase().as_str() {
      "COM" => ArgMMods::COM,
      "LOC" => ArgMMods::LOC,
      "DIR" => ArgMMods::DIR,
      "GOL" => ArgMMods::GOL,
      "MNR" => ArgMMods::MNR,
      "TMP" => ArgMMods::TMP,
      "EXT" => ArgMMods::EXT,
      "REC" => ArgMMods::REC,
      "PRD" => ArgMMods::PRD,
      "PRP" => ArgMMods::PRP,
      "CAU" => ArgMMods::CAU,
      "DIS" => ArgMMods::DIS,
      "ADV" => ArgMMods::ADV,
      "ADJ" => ArgMMods::ADJ,
      "MOD" => ArgMMods::MOD,
      "NEG" => ArgMMods::NEG,
      "DSP" => ArgMMods::DSP,
      "LVB" => ArgMMods::LVB,
      "CXN" => ArgMMods::CXN,
      _ => ArgMMods::Unknown,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
  pub action_type: String, // propbank predicates.roleset.id for now...

  pub symbol: String, // maybe this should be "symbol"? 

  pub properties: Vec<ActionProperties>,
}

impl Action {
  pub fn new(action_type: String, symbol: &mut Symbol) -> Self {
    Self {
      action_type,
      symbol: symbol.next_symbol(),
      properties: Vec::new(),
    }
  }

  pub fn from_lemma(lemma: &str) -> Self {
    let l = lemma.to_lowercase();
    Action {
      action_type: l.clone(),
      symbol: format!("${l}"),
      properties: vec![],
    }
  }

  pub fn get_symbol(&self) -> String {
    self.symbol.clone()
  }
}

/*
ArgO - agent
Arg1 - patient
Arg2 - instrument, benefactive, attribute
Arg3 - starting point, benefactive, attribute
Arg4 - ending point
ArgM - modfier
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "action_property_type")]
pub enum ActionProperties {
  Agent { agent: String },
  Patient { patient: String },
  Instrument { instrument: String },
  Benefactive { benefactive: String },
  Outcome { outcome: String },
  Recipient { recipient: String },
  Direction { direction: Directions },
  Attribute { attribute: String },
  Purpose { purpose: String }, // symbol to an action or event
  Negate { negate: bool },
}