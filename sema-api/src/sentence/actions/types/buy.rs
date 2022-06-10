use crate::sentence::actions::action_types::ActionTypes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyAction {
  action_type: ActionTypes, // should always be ActionTypes::Buy
  handle: String,
  buyer: String, // Arg0: buyer (vnrole: 13.5.1-agent)
  entities: Vec<String>, // Arg1: things bought (vnrole: 13.5.1-theme)
}

impl BuyAction {
  pub fn new(handle: String, buyer: String, entities: Vec<String>) -> Self {
    BuyAction {
      action_type: ActionTypes::Buy,
      handle,
      buyer,
      entities,
    }
  }
}