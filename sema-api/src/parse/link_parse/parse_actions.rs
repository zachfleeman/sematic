use anyhow::Result;

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{action::{Action, ActionProperties}, sema_sentence::SemaSentence, symbol::Symbol},
};

use link_parser_rust_bindings::{pos::POS, lp::{disjunct::ConnectorPointing, link_types::LinkTypes}};

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
    // "N" connects the word "not" to preceding verb. "he DID NOT go"
    // .filter(|w| !w.has_raw_disjunct("I+") && !w.has_raw_disjunct("N+")) 
    {

    // Infinitives can be verbs. Don't know if this is for all cases
    // Will handle later.
    if v.has_disjunct(LinkTypes::I, ConnectorPointing::Right) {
      continue;
    }
    // let mut action_type = v.get_cleaned_word();
    let mut action_type = part.get_word_lemma(&v);

    // Need to use the lemma of the verb if it's past tense.
    if matches!(v.pos, Some(POS::VerbPastTense | POS::VerbPastTense2)) {
      action_type = part.get_word_lemma(&v);
    }

    let mut action = Action::new(action_type, symbol);

    // handle adverbs
    if v.has_disjunct(LinkTypes::E, ConnectorPointing::Left) {
      // the verb has some kind of adverb-y thing attached to it.
      if let Some(prev_word) = part.links.get_prev_word(&v) {
        if prev_word.has_raw_disjunct("En+") {
          // the adverb is "not"
          action.properties.push(ActionProperties::Negate { negate: true });
        }
      }
    }

    parse_state.add_symbol(&action.symbol, vec![v.position]);
    
    actions.push(action);
  }

  repaired_sentence
    .actions
    .clear();

  repaired_sentence.actions = actions;

  Ok(repaired_sentence)
}
