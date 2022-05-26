use super::{
  action::Action, agents::Agents, entity::Entity, event::Event, location::Locations,
  query::Queries, relation::Relations, temporal::Temporals,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemaSentence {
  pub agents: Vec<Agents>,

  pub entities: Vec<Entity>,

  pub locations: Vec<Locations>,

  pub temporal: Vec<Temporals>,

  pub relations: Vec<Relations>,

  pub actions: Vec<Action>,

  pub events: Vec<Event>,

  pub queries: Vec<Queries>,
}

impl SemaSentence {
  pub fn new() -> Self {
    Self {
      agents: Vec::new(),
      entities: Vec::new(),
      locations: Vec::new(),
      temporal: Vec::new(),
      relations: Vec::new(),
      actions: Vec::new(),
      events: Vec::new(),
      queries: Vec::new(),
    }
  }
}
