use crate::sentence::actions::{action_types::ActionTypes};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAction {
  action_type: ActionTypes, // should always be ActionTypes::Create
  handle: String,
  entities: Vec<String>,
  multiplier: usize, // would it be possible to have fractional multipliers?
}

impl CreateAction {
  pub fn new(handle: String, entities: Vec<String>, multiplier: usize) -> Self {
    CreateAction {
      action_type: ActionTypes::Create,
      handle,
      entities,
      multiplier,
    }
  }
}
