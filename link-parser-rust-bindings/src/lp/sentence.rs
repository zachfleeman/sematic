use anyhow::Result;
use std::ops::Range;

use super::{
  disjunct::{ConnectorPointing, Disjunct, FreeWordOrder},
  link_types::LinkTypes,
  word::Word,
};
use crate::pos::POS;
use serde::{Deserialize, Serialize};

pub type WordDisjunctsPair = Vec<(String, Vec<String>, Range<u64>, Range<u64>)>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sentence {
  pub original_sentence: String,
  pub words: Vec<Word>,
}

impl Sentence {
  pub fn new(sentence: &str, word_disjuncts_pairs: WordDisjunctsPair) -> Self {
    let words = word_disjuncts_pairs
      .into_iter()
      .enumerate()
      .map(|(position, (word, disjuncts_strs, bytes, chars))| {
        let disjuncts = disjuncts_strs
          .into_iter()
          .map(|disjunct| Disjunct::new(&disjunct))
          .collect::<Vec<Disjunct>>();

        Word::new(position, &word, disjuncts, bytes, chars)
      })
      .collect::<Vec<Word>>();

    Sentence {
      original_sentence: sentence.to_owned(),
      words,
    }
  }

  pub fn get_verbs(&self) -> Vec<Word> {
    let verbs = self
      .words
      .iter()
      .filter(|word| {
        if let Some(pos) = &word.pos {
          pos == &POS::Verb
            || pos == &POS::Verb2
            || pos == &POS::VerbPastTense
            || pos == &POS::VerbPastTense2
        } else {
          false
        }
      })
      .map(|w| w.to_owned())
      .collect::<Vec<Word>>();

    verbs
  }

  pub fn get_past_tense_verbs(&self) -> Vec<Word> {
    let verbs = self
      .words
      .iter()
      .filter(|word| {
        if let Some(pos) = &word.pos {
          pos == &POS::VerbPastTense
            || pos == &POS::VerbPastTense2
        } else {
          false
        }
      })
      .map(|w| w.to_owned())
      .collect::<Vec<Word>>();

    verbs
  }

  // TODO: This should return a Vec or Slice of words, because are often multiple words. e.g "pivot table"
  pub fn get_known_nouns(&self) -> Vec<Word> {
    let nouns = self
      .words
      .iter()
      .filter_map(|lp_word| {
        if let Some(pos) = &lp_word.pos {
          match pos {
            POS::GivenName => None,
            POS::CurrencyName => None, // .b
            POS::BinaryOperator => None,
            POS::GivenNameFeminine => None, // .f
            POS::Gerund => None,
            POS::Identifier => None,
            POS::Interjection => None,
            // POS::Conjunction => None,
            // POS::ConjunctionAdjective => None,
            // POS::ConjunctionComparative => None,
            // POS::ConjunctionProperName => None,
            // POS::ConjunctionPostNominalModifier => None,
            // POS::ConjunctionNoun => None,
            // POS::ConjunctionDitransitive => None,
            // POS::ClauseOpener => None,
            // POS::ConjunctionQuestionWords => None,
            // POS::ConjunctionsAdPrep => None,
            // POS::ConjunctionInterval => None,
            // POS::ConjunctionNumericalSum => None,
            // POS::ConjunctionVerb => None,
            POS::NotUsed => None,
            POS::Location => None,
            POS::GivenNameMasculine => None, // .m
            POS::Noun => Some(lp_word.to_owned()),
            POS::NounUncountable => Some(lp_word.to_owned()),
            POS::Organization => None,
            POS::OrdinalNumber => None,
            // POS::PluralCountNoun => Some(lp_word.to_owned()),
            POS::SingularMassNoun => Some(lp_word.to_owned()),
            POS::Title => None,
            POS::TimeDate => None,
            POS::TimeZone => None,
            POS::UnitOfMeasurement => None,
            POS::PrefixAbbreviation => None,
            POS::PostfixAbbreviation => None,
            POS::NotUsed2 => None,
            _ => None,
          }
        } else {
          None
        }
      })
      .collect::<Vec<Word>>();

    nouns
  }

  pub fn get_word_by_position(&self, position: usize) -> Option<&Word> {
    self
      .words
      .iter()
      .find(|w| w.position == position)
  }

  pub fn is_empty(&self) -> bool {
    self.words.is_empty()
  }

  pub fn get_prev_word(&self, word: &Word) -> Option<&Word> {
    let prev_word_position = word.position - 1;
    self.get_word_by_position(prev_word_position)
  }

  pub fn get_prev_words(&self, word: &Word) -> Vec<&Word> {
    self.words[..word.position].iter().collect::<Vec<&Word>>()
  }

  pub fn get_next_word(&self, word: &Word) -> Option<&Word> {
    let next_word_position = word.position + 1;
    self.get_word_by_position(next_word_position)
  }

  pub fn get_next_words(&self, word: &Word) -> Vec<&Word> {
    self.words[word.position..].iter().collect::<Vec<&Word>>()
  }

  pub fn get_next_words_skip(&self, word: &Word, skip: usize) -> Vec<&Word> {
    let skip = word.position + skip;
    self.words[skip..].iter().collect::<Vec<&Word>>()
  }

  pub fn find_prev_word_with_link(&self, word: &Word, link_type: LinkTypes, conn_pointing: ConnectorPointing) -> Option<&Word> {
    self.words[..word.position].iter().rev().find(|w| w.has_disjunct(link_type, conn_pointing))
  }

  pub fn find_next_word_with_link(&self, word: &Word, link_type: LinkTypes, conn_pointing: ConnectorPointing) -> Option<&Word> {
    self.words[word.position..].iter().find(|w| w.has_disjunct(link_type, conn_pointing))
  }

  pub fn find_prev_word_with_disjunct(&self, word: &Word, link_type: LinkTypes, conn_pointing: ConnectorPointing, prescript: FreeWordOrder) -> Option<&Word> {
    self.words[..word.position].iter().rev().find(|w| w.has_disjunct_with_prescript(link_type, conn_pointing, prescript))
  }

  pub fn find_next_word_with_disjunct(&self, word: &Word, link_type: LinkTypes, conn_pointing: ConnectorPointing, prescript: FreeWordOrder) -> Option<&Word> {
    self.words[word.position..].iter().find(|w| w.has_disjunct_with_prescript(link_type, conn_pointing, prescript))
  }

  pub fn find_prev_word_with_raw_disjunct(&self, word: &Word, disjunct: &str) -> Option<&Word> {
    self.words[..word.position].iter().rev().find(|w| w.has_raw_disjunct(disjunct))
  }

  pub fn find_next_word_with_raw_disjunct(&self, word: &Word, disjunct: &str) -> Option<&Word> {
    self.words[word.position..].iter().find(|w| w.has_raw_disjunct(disjunct))
  }
}


pub fn get_a_left_words(
  word: &Word,
  words: &Vec<Word>,
  _disjunct: &Disjunct,
) -> Result<Vec<Word>> {
  let mut a_link_words = vec![];

  for word in words[0..word.position].iter().rev() {
    if word.has_disjunct(LinkTypes::A, ConnectorPointing::Right) {
      a_link_words.push(word.to_owned());
    }
  };

  Ok(a_link_words)
}

pub fn get_s_left_words(
  word: &Word,
  words: &Vec<Word>,
  _disjunct: &Disjunct,
) -> Result<Vec<Word>> {
  let mut s_link_words = vec![];

  for word in words[0..word.position].iter().rev() {
    if word.has_disjunct(LinkTypes::S, ConnectorPointing::Right) {
      s_link_words.push(word.to_owned());
    }
  };

  Ok(s_link_words)
}