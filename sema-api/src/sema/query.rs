#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "query_type")]
pub enum Queries {
  Subject(Subject),
  Query(Query)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
  pub symbol: String,
  pub properties: Vec<QueryProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryProperties {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
  pub symbol: String,
  pub properties: Vec<SubjectProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectProperties {}