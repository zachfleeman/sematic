use anyhow::Result;

use link_parser_rust_bindings::lp::word::Word;

use crate::{
  nlp::{duck::DuckValues, sentence_parts::SentenceParts},
  parse::numbers::construct_number,
  sema::{
    sema_sentence::SemaSentence,
    symbol::Symbol,
    query::{Queries, Query, QueryProperties},
  },
};

use super::link_parse::ParseState;

pub fn parse_queries(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState
) -> Result<SemaSentence> {
  let mut output_sentence = sema_sentence.clone();

  // sentence is a question
  // can't rely on all questions having a question mark
  let is_question = part.links.is_question();
  dbg!(&is_question);
  
  if is_question {
    let q = Query {
      symbol: symbol.next_symbol(),
      properties: vec![],
    };

    output_sentence.queries.push(Queries::Query(q));
  }

  Ok(output_sentence)
}