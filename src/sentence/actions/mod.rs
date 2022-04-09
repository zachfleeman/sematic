pub use self::action_types::ActionTypes;

use anyhow::Result;

pub mod action_types;
pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actions {
  Create(types::CreateAction),
  Buy(types::BuyAction),
}

pub async fn get_action() -> Result<ActionTypes> {
  Ok(ActionTypes::Create)
}