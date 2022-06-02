use anyhow::Result;

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    action::{Action},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

pub fn parse_actions(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  let verbs = part
    .links
    .get_verbs();

  let actions = verbs
    .into_iter()
    .map(|verb| Action::new(verb.word.clone(), symbol))
    .collect::<Vec<Action>>();

  repaired_sentence
    .actions
    .clear();

  repaired_sentence.actions = actions;

  Ok(repaired_sentence)
}