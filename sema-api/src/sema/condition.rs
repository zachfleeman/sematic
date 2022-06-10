use fake::faker::address::en::*;
use fake::{Dummy, Fake, Faker};

use rand::Rng;

use std::string::ToString;
use strum_macros::{self, Display};

use crate::data_gen::gen_helpers::*;

use super::helpers::to_symbol;
use super::symbol::Symbol;

/*
Check out the MongoDB docs for more info on "Comparison Query Operators"
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "condition_type")]
pub enum Conditions {
  Equals(Equals),
  NotEquals(NotEquals),
  GreaterThan(GreaterThan),
  LessThan(LessThan),
  GreaterThanOrEqual(GreaterThanOrEqual),
  LessThanOrEqual(LessThanOrEqual),
  In(In),
  NotIn(NotIn),
  // While?
}

pub enum ConditionProperties {
  Agent { agent: String },
  AgentProperty { agent: String, agent_property: Option<String> },
  Entity { entity: String },
  EntityProperty { entity: String, entity_property: Option<String> },
  Location { location: String },
  LocationProperty { location: String, location_property: Option<String> },
  Temporal { temporal: String },
  TemporalProperty { temporal: String, temporal_property: Option<String> },
  Relation { relation: String },
  RelationProperty { relation: String, relation_property: Option<String> },
  Action { action: String },
  ActionProperty { action: String, action_property: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equals {
  pub symbol: Symbol,
  pub properties: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreaterThan {
  pub symbol: Symbol,
  pub lhs: Vec<ConditionProperties>,
  pub rhs: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessThan {
  pub symbol: Symbol,
  pub lhs: Vec<ConditionProperties>,
  pub rhs: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreaterThanOrEqual {
  pub symbol: Symbol,
  pub properties: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessThanOrEqual {
  pub symbol: Symbol,
  pub properties: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct In {
  pub symbol: Symbol,
  pub properties: Vec<ConditionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotIn {
  pub symbol: Symbol,
  pub properties: Vec<ConditionProperties>,
}