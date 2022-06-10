use anyhow::Result;

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{action::Action, sema_sentence::SemaSentence, symbol::Symbol},
};

use super::link_parse::ParseState;

pub fn parse_actions(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  let verbs = part
    .links
    .get_verbs();

  let mut actions = vec![];

  for v in verbs
    .into_iter()
    .filter(|w| !w.has_raw_disjunct("I+")) // Infinitives can be verbs. Don't know if this is for all cases
  {
    let action = Action::new(v.word.clone(), symbol);
    parse_state.add_symbol(&action.symbol, vec![v.position]);
    actions.push(action);
  }

  repaired_sentence
    .actions
    .clear();

  repaired_sentence.actions = actions;

  Ok(repaired_sentence)
}
