use anyhow::Result;
use std::collections::HashMap;

use super::{
  parse_actions::parse_actions, parse_agents::parse_agents, parse_entities::parse_entities,
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{sema_sentence::SemaSentence, symbol::Symbol},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseState {
  // key: symbol, eg. "$1"
  // value: the positions of the words that the symbol references, eg. [0, 1, 2]
  pub symbol_positions: HashMap<String, Vec<usize>>,
}

impl ParseState {
  pub fn new() -> Self {
    Self {
      symbol_positions: HashMap::new(),
    }
  }

  pub fn add_symbol(&mut self, symbol: &str, positions: Vec<usize>) {
    self
      .symbol_positions
      .insert(symbol.to_string(), positions);
  }

  pub fn get_symbol_positions(&self, symbol: &str) -> Option<&Vec<usize>> {
    self
      .symbol_positions
      .get(symbol)
  }
}

pub fn parse_with_links(part: SentenceParts) -> Result<Option<SemaSentence>> {
  let mut symbol = Symbol::new(0);
  let mut parse_state = ParseState::new();
  let mut sema_sentence = SemaSentence::new();

  parse_agents(&mut sema_sentence, &part, &mut symbol, &mut parse_state)?;

  // let sema_sentence = parse_actions(&sema_sentence, &part, &mut symbol)?;

  // let sema_sentence = parse_entities(&sema_sentence, &part, &mut symbol)?;

  // // Connect up all the objects created earlier
  // let sema_sentence = connect_actions(&sema_sentence, &part, &mut symbol)?;

  dbg!(&parse_state);

  Ok(Some(sema_sentence))
}

pub fn connect_actions(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
) -> Result<SemaSentence> {
  // Agent / Arg 0

  Ok(sema_sentence.to_owned())
}
