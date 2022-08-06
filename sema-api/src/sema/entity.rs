use convert_case::{Case, Casing};

use super::symbol::Symbol;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
  pub entity_type: String,

  pub symbol: String,

  pub properties: Vec<EntityProperties>,
}

impl Entity {
  pub fn new(object_type: String, symbol: &mut Symbol) -> Self {
    let ot = object_type.to_case(Case::Snake);

    Self {
      entity_type: ot.clone(),
      symbol: symbol.next_symbol(),
      properties: Vec::new(),
    }
  }

  pub fn get_symbol(&self) -> String {
    self.symbol.clone()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "entity_property_type")]
#[serde(untagged)]
pub enum EntityProperties {
  Modifier {
    modifier_type: String, // category (chromatic)
    modifier: Option<String>, // actual language used (colored)
    amplifiers: Vec<String>, // like satellites (hot pink)
  },
  Count { count: f32 },
  Quantity { quantity: Quantities },
  Occurance { occurs: String }, // symbol to temporal
  Attribute { attribute: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Quantities {
  All,
  None,
  Some,
  Few,
  Many,
  Most,
  Multiple,
  Both,
  Single,
}

impl EntityProperties {
  pub fn new_modifier(modifier_type: String, modifier: Option<String>) -> Self {
    Self::Modifier {
      modifier_type: modifier_type.to_case(Case::Snake),
      modifier,
      amplifiers: Vec::new(),
    }
  }
}
