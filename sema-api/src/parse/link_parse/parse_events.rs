use anyhow::Result;

// use link_parser_rust_bindings::{
//   lp::{disjunct::ConnectorPointing, link_types::LinkTypes, word::Word},
//   pos::POS,
// };

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    // event::{Event, EventProperties},
    // entity::{Entity, EntityProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;

pub fn parse_events(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  _symbol: &mut Symbol,
  _parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let output_sentence = sema_sentence.clone();

  // Past tense verbs require an event.
  let _past_tense_verbs = part
    .links
    .get_past_tense_verbs();

  /*
  First off need to create an event for actions(verbs) that are in the past tense.
  Also, maybe the future tense as well?
   */
  Ok(output_sentence)
}