#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifierTense {
  Present,
  Past,
  Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityModifer {
  pub modifier_type: String, // EntityModifierTypes,
  pub handle: String,
  pub tense: ModifierTense,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
  pub entity_type: String, //EntityTypes,
  pub handle: String,
  pub modifiers: Vec<EntityModifer>,
}