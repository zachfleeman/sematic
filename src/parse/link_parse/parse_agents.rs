use anyhow::Result;

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    agents::{Agents, Ego},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;

pub fn parse_agents(
  sema_sentence: &mut SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<()> {
  // "I" and "My" detection
  let ego_words = part
    .links
    .words
    .iter()
    .filter(|lp_word| {
      lp_word
        .word
        .to_lowercase()
        == "i"
        || lp_word
          .word
          .to_lowercase()
          == "my"
    })
    .collect::<Vec<_>>();

  if ego_words.len() > 0 && !sema_sentence.has_ego_agent() {
    let ego = Ego::new(symbol);

    let ego_positions = ego_words
      .iter()
      .map(|lp_word| lp_word.position)
      .collect::<Vec<_>>();

    parse_state.add_symbol(&ego.symbol, ego_positions);

    let agent = Agents::Ego(ego);

    sema_sentence
      .agents
      .push(agent);
  }

  Ok(())
}
