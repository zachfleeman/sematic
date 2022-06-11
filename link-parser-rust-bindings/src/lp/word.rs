// use nlprule::types::Sentence;
use serde::{Deserialize, Serialize};

use crate::{lp::disjunct::Disjunct, pos::POS};

use super::disjunct::ConnectorPointing;
use super::link_types::LinkTypes;

// use nlprule::types::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
  lemma: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
  pub position: usize,
  pub word: String,
  pub pos: Option<POS>,
  pub disjuncts: Vec<Disjunct>, // spliting for now, but should be able to do more in the future.
  pub morpho_guessed: bool,
  pub unknown_word: bool,
  pub year_date: bool,
  pub day_ordinals: bool,
}

impl Word {
  pub fn new(
    position: usize,
    lp_word: &str,
    disjuncts: Vec<Disjunct>, // tokens: Sentence, // token sentence
  ) -> Self {
    let word = match lp_word {
      "." => ".".to_string(),
      _ => lp_word
        .split(".")
        .nth(0)
        .unwrap()
        .to_string(),
    };

    let pos = POS::from_lp_word(lp_word);

    let morpho_guessed = word.contains("[!");
    let unknown_word = word.contains("[?]");

    let year_date = word.contains("[!<YEAR-DATE>]");
    let day_ordinals = word.contains("[!<DAY-ORDINALS>]");

    Word {
      position,
      word,
      disjuncts,
      pos,
      morpho_guessed,
      unknown_word,
      year_date,
      day_ordinals,
    }
  }

  pub fn has_pos(&self, test_pos: POS) -> bool {
    match self.pos {
      Some(pos) => pos == test_pos,
      None => false,
    }
  }

  pub fn has_disjunct(&self, link_type: LinkTypes, conn_pointing: ConnectorPointing) -> bool {
    self
      .disjuncts
      .iter()
      .any(|disjunct| disjunct.pointing == conn_pointing && link_type.eq(&disjunct.link_type))
  }

  pub fn has_raw_disjunct(&self, raw_disjunct: &str) -> bool {
    self
      .disjuncts
      .iter()
      .any(|disjunct| disjunct.raw == raw_disjunct)
  }

  pub fn get_raw_disjuncts(&self) -> Vec<String> {
    self
      .disjuncts
      .iter()
      .map(|disjunct| disjunct.raw.clone())
      .collect::<Vec<String>>()
  }

  pub fn get_cleaned_word(&self) -> String {
    self
      .word
      .split("[")
      .nth(0)
      .unwrap()
      .to_string()
  }

  pub fn word_is_capitalized(&self) -> bool {
    if !matches!(self.pos, Some(POS::LeftWall) | Some(POS::RightWall)) {
      self
        .word
        .chars()
        .next()
        .unwrap()
        .is_uppercase()
    } else {
      false
    }
  }
}
