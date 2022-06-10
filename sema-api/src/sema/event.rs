use convert_case::{Casing, Case};

use super::{symbol::Symbol, temporal::{Temporals, Tense}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
  pub event_type: String,

  pub symbol: String,

  pub properties: Vec<EventProperties>,
}

impl Event {
  pub fn new(event_type: String, symbol: &mut Symbol) -> Self {
    let evt_type = event_type.to_case(Case::Snake);
    Self {
      event_type: evt_type.clone(),
      symbol: symbol.next_symbol(),
      properties: Vec::new(),
    }
  }

  pub fn get_symbol(&self) -> String {
    self.event_type.clone()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "event_property_type")]
#[serde(untagged)]
pub enum EventProperties {
  Action { action: String },
  Occurance { occurs: String }, // symbol to temporal
  Duration { duration: String },  // symbol to temporal that is of type duration
  Tense { tense: Tense },
  Location { location: String } // might want to give location as a property for actions too. 
}

pub enum Interval {
  Once,
  Repeating(Temporals),
  // start and end?
}