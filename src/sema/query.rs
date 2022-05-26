#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "agent_type")]
pub enum Queries {
  Query(Query)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
  pub symbol: String,
  pub properties: Vec<QueryProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryProperties {}