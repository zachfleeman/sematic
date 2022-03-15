use self::action_types::ActionTypes;

use anyhow::Result;

pub mod action_types;
pub mod types;

pub enum Actions {
  Create(types::CreateAction),
}

pub async fn get_action() -> Result<ActionTypes> {
  Ok(ActionTypes::Create)
}