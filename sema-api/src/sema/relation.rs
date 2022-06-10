#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "relation_type")]
pub enum Relations {
  Ownership(Ownership),
  Origin(Origin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ownership {
  pub symbol: String,
  pub properties: Vec<OwnershipProperties>,
}

impl Ownership {
  pub fn new() -> Self {
    Self {
      symbol: "$ownership".to_string(),
      properties: vec![],
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "ownership_property_type")]
pub enum OwnershipProperties {
  Owner { owner: String },
  Owned { owned: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
  pub symbol: String,
  pub properties: Vec<OriginProperties>,
}

impl Origin {
  pub fn new() -> Self {
    Self {
      symbol: "$origin".to_string(),
      properties: vec![],
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "origin_property_type")]
pub enum OriginProperties {
  Origin { origin: String },
  Entity { entity: String },
  Agent { agent: String },
  Location { location: String },
  Event { event: String },
}