// use anyhow::Result;

// use crate::lp::word::LPWord;
// use crate::lp::links::{self, Plurality};

use super::link_types::LinkTypes;

// use links::*;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectorPointing {
  Left,
  Right,
}

impl ConnectorPointing {
  pub fn from_raw_disjunct(disjunct: &str) -> Self {
    match disjunct
      .chars()
      .last()
    {
      Some('-') => ConnectorPointing::Left,
      Some('+') => ConnectorPointing::Right,
      _ => panic!("Invalid disjunct: {}", disjunct),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscript {
  pub letter: char,
}

pub fn get_disjunct_subscripts(disjunct: &str) -> Vec<Subscript> {
  let d = if disjunct.starts_with(|c: char| c.is_lowercase()) {
    &disjunct[1..]
  } else {
    disjunct
  };

  d.chars()
  .filter(|c| c.is_lowercase())
  .map(|c| Subscript { letter: c })
  .collect::<Vec<Subscript>>()
}

pub fn is_disjunct_multiple(disjunct: &str) -> bool {
  match disjunct.chars().nth(0) {
    Some('@') => true,
    _ => false,
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum FreeWordOrder {
  Head, // h prescript, 
  Dependent // d prescript
}

pub fn get_disjunct_prescript(disjunct: &str) -> Option<FreeWordOrder> {
  match disjunct.chars().nth(0) {
    Some('h') => Some(FreeWordOrder::Head),
    Some('d') => Some(FreeWordOrder::Dependent),
    _ => None,
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disjunct {
  pub raw: String,
  pub pointing: ConnectorPointing,
  pub link_type: LinkTypes,
  pub subscripts: Vec<Subscript>,
  pub prescript: Option<FreeWordOrder>,
  pub multiple: bool,
}

impl Disjunct {
  pub fn new(disjunct: &str) -> Self {
    Disjunct {
      raw: disjunct.to_string(),
      pointing: ConnectorPointing::from_raw_disjunct(disjunct),
      link_type: LinkTypes::from_disjunt_str(disjunct),
      subscripts: get_disjunct_subscripts(disjunct),
      prescript: get_disjunct_prescript(disjunct),
      multiple: is_disjunct_multiple(disjunct),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
  source_index: usize,
  target_indices: Vec<usize>,
}
