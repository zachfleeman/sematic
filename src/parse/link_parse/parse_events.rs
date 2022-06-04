use anyhow::Result;

use link_parser_rust_bindings::{
  lp::{disjunct::ConnectorPointing, link_types::LinkTypes, word::Word},
  pos::POS,
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    event::{Event, EventProperties},
    // entity::{Entity, EntityProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;

pub fn parse_events(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut output_sentence = sema_sentence.clone();

  /*
  First off need to create an event for actions(verbs) that are in the past tense.
  Also, maybe the future tense as well?
   */
  Ok(output_sentence)
}