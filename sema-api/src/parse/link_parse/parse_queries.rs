use anyhow::Result;

use link_parser_rust_bindings::lp::{
  disjunct::ConnectorPointing, link_types::LinkTypes, word::Word,
};

use crate::{
  nlp::{duck::DuckValues, sentence_parts::SentenceParts},
  parse::numbers::construct_number,
  sema::{
    query::{Queries, Query, QueryProperties, Subject, SubjectProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;

pub fn parse_queries(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut output_sentence = sema_sentence.clone();

  // sentence is a question
  // can't rely on all questions having a question mark
  let is_question = part
    .links
    .is_question();
  dbg!(&is_question);

  if is_question {
    let w_disjunct_option = part
      .links
      .get_left_wall()
      .and_then(|lw| lw.get_disjunct(LinkTypes::W, ConnectorPointing::Right));

    if let Some(w_disjunct) = w_disjunct_option {
      if w_disjunct.has_subscript(vec!["s"]) {
        let s = Subject {
          symbol: symbol.next_symbol(),
          properties: vec![],
        };

        output_sentence
          .queries
          .push(Queries::Subject(s));
      }
    } else {
      let q = Query {
        symbol: symbol.next_symbol(),
        properties: vec![],
      };

      output_sentence
        .queries
        .push(Queries::Query(q));
    }
  }

  Ok(output_sentence)
}
