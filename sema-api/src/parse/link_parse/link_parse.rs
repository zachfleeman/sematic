use anyhow::Result;
use link_parser_rust_bindings::lp::{
  disjunct::{ConnectorPointing, FreeWordOrder},
  link_types::LinkTypes,
  word::Word,
};
use std::collections::HashMap;

use super::{
  parse_actions::parse_actions, parse_agents::parse_agents, parse_entities::parse_entities,
  parse_events::parse_events, parse_queries::parse_queries, parse_temporal::parse_temporal,
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  // parse,
  sema::{action::ActionProperties, sema_sentence::SemaSentence, symbol::Symbol},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseState {
  // key: symbol, eg. "$1"
  // value: the positions of the words that the symbol references, eg. [0, 1, 2]
  pub symbol_positions: HashMap<String, Vec<usize>>,
}

impl ParseState {
  pub fn new() -> Self {
    Self {
      symbol_positions: HashMap::new(),
    }
  }

  pub fn add_symbol(&mut self, symbol: &str, positions: Vec<usize>) {
    self
      .symbol_positions
      .insert(symbol.to_string(), positions);
  }

  pub fn get_symbol_positions(&self, symbol: &str) -> Option<&Vec<usize>> {
    self
      .symbol_positions
      .get(symbol)
  }

  pub fn get_symbols_by_position(&self, position: usize) -> Vec<String> {
    let mut symbols = vec![];

    for (symbol, positions) in &self.symbol_positions {
      if positions.contains(&position) {
        symbols.push(symbol.to_string());
      }
    }

    symbols
  }
}

pub fn parse_with_links(part: SentenceParts) -> Result<Option<SemaSentence>> {
  let mut symbol = Symbol::new(0);
  let mut parse_state = ParseState::new();
  let mut sema_sentence = SemaSentence::new();

  parse_agents(&mut sema_sentence, &part, &mut symbol, &mut parse_state)?;

  let sema_sentence = parse_actions(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  let sema_sentence = parse_entities(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  let sema_sentence = parse_temporal(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  let sema_sentence = parse_events(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  let sema_sentence = parse_queries(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  // // Connect up all the objects created earlier
  let sema_sentence = connect_actions(&sema_sentence, &part, &mut symbol, &mut parse_state)?;

  // dbg!(&sema_sentence);

  Ok(Some(sema_sentence))
}

pub fn connect_actions(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  _symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut action_connected_sentence = sema_sentence.clone();

  for action in action_connected_sentence
    .actions
    .iter_mut()
  {
    // Agent / Arg 0
    let positions = parse_state
      .get_symbol_positions(&action.symbol)
      .expect("parse_actions should have created an entry in the parse_state symbols");

    let action_words = positions
      .iter()
      .filter_map(|p| {
        part
          .links
          .get_word_by_position(*p)
      })
      .collect::<Vec<&Word>>();

    for aw in action_words.into_iter() {
      // determine Agent / Arg 0 links
      // Are there any S links (noun to verb): https://www.abisource.com/projects/link-grammar/dict/section-S.html
      if aw.has_disjunct(LinkTypes::S, ConnectorPointing::Left) {
        // Left Pointing S link exists, which means that the noun has an Arg 0/Agent link

        for word in part.links.words[0..aw.position]
          .iter()
          .rev()
        {
          if word.has_disjunct(LinkTypes::S, ConnectorPointing::Right) {
            // Right Pointing S link exists, which means that the noun has an Arg 0/Agent link
            // TODO: this is not true in a sentence like "the cat was chased by the dog"
            parse_state
              .get_symbols_by_position(word.position)
              .iter()
              .for_each(|s| {
                action
                  .properties
                  .push(ActionProperties::Agent {
                    agent: s.to_owned(),
                  });
              });

            break; // only one S link per action for now, but need to handle "p" subscripts
          }
        }
      }

      if aw.has_disjunct(LinkTypes::I, ConnectorPointing::Left) {
        // Left Pointing I means the the verb has an infinitive (will/must/etc) to the left of it.
        // Need to still find the agent in a situation like "I will chase the cat".
        part
          .links
          .find_prev_word_with_link(&aw, LinkTypes::I, ConnectorPointing::Right)
          .and_then(|i| {
            if i.has_disjunct(LinkTypes::S, ConnectorPointing::Left) {
              part
                .links
                .find_prev_word_with_link(&i, LinkTypes::S, ConnectorPointing::Right)
            } else {
              None
            }
          })
          .and_then(|s| {
            parse_state
              .get_symbols_by_position(s.position)
              .iter()
              .for_each(|ss| {
                action
                  .properties
                  .push(ActionProperties::Agent {
                    agent: ss.to_owned(),
                  });
              });

            Some(())
          });
      }

      if aw.has_disjunct(LinkTypes::IV, ConnectorPointing::Right) {
        part
          .links
          .find_next_word_with_link(&aw, LinkTypes::IV, ConnectorPointing::Left)
          .and_then(|iv| {
            parse_state
              .get_symbols_by_position(iv.position)
              .iter()
              .for_each(|s| {
                action
                  .properties
                  .push(ActionProperties::Outcome {
                    outcome: s.to_owned(),
                  });
              });

            Some(())
          });
      }

      // "O" connects transitive verbs to their objects, direct or indirect: "She SAW ME", "I GAVE HIM the BOOK".
      if aw.has_disjunct(LinkTypes::O, ConnectorPointing::Right) {
        // Verbs can have two right pointing links. e.g O+ O*n+
        // How to handle this?
        // Right Pointing O link exists, which means that the verb has an Arg 1/Object link
        let aw_o_disjuncts = aw.get_disjuncts(LinkTypes::O);

        match aw_o_disjuncts.len() {
          1 => {
            for word in part.links.words[aw.position + 1..].iter() {
              if word.has_disjunct(LinkTypes::O, ConnectorPointing::Left) {
                // Left Pointing O link exists, which means that the verb has an Arg 1/Object link

                // if word is a single (has subscript of "s")
                parse_state
                  .get_symbols_by_position(word.position)
                  .iter()
                  .for_each(|s| {
                    action
                      .properties
                      .push(ActionProperties::Patient {
                        patient: s.to_owned(),
                      });
                  });
              }
            }
          }
          2 => {
            // One O link is to the patient, and other is to a benefactive, I think
            /*
            from: https://www.abisource.com/projects/link-grammar/dict/section-O.html
            "Other verbs have two O+ connectors, one or both of which may
            be optional ("I gave him five dollars", "I gave five
            dollars"). In this case, the first object may either be a
            pronoun or a noun; however, if it is a noun, the second may
            not be a pronoun: "I gave him the money", "I gave Jane the
            money", "*I gave Jane it", "*I gave him it". This is parallel
            to the case of particles; in transitive verbs which take
            particles like "up" or "out", the particle may not precede a
            pronoun ("*We sorted out them").  The O*n+/Ox- subscripts,
            developed for that purpose, are used here as well. The second
            O+ connector on two-object verbs has O*n+; pronouns have Ox-;
            "Oxn" is prohibited in post-processing. (See "K".)""
            */
            // Pronouns have Ox-, but not Oxn
            // Second O+ connector on two-object verbs has O*n+; pronouns have Ox-;

            let first_o = part
              .links
              .get_next_words(aw)
              .into_iter()
              .find(|w| w.has_disjunct(LinkTypes::O, ConnectorPointing::Left))
              .expect("No first left pointing O link found");

            if first_o.has_raw_disjunct("Ox-") {
              // first O is a pronoun (me, she, I, it, etc)
              parse_state
                .get_symbols_by_position(first_o.position)
                .iter()
                .for_each(|s| {
                  action
                    .properties
                    .push(ActionProperties::Benefactive {
                      benefactive: s.to_owned(),
                    });
                });
            }

            let second_o = part
              .links
              .get_next_words_skip(first_o, 1)
              .into_iter()
              .find(|w| w.has_disjunct(LinkTypes::O, ConnectorPointing::Left))
              .expect("No Second left pointing O link found");

            let second_o_symbols = parse_state.get_symbols_by_position(second_o.position);

            // TODO: not sure yet how best to handle single/plural/multiple, but the info is encoded in the links.
            // single
            if second_o.has_raw_disjunct("Os-") {
              second_o_symbols
                .iter()
                .for_each(|s| {
                  action
                    .properties
                    .push(ActionProperties::Patient {
                      patient: s.to_owned(),
                    });
                });
            }

            // plural
            if second_o.has_raw_disjunct("Op-") {
              second_o_symbols
                .iter()
                .for_each(|s| {
                  action
                    .properties
                    .push(ActionProperties::Patient {
                      patient: s.to_owned(),
                    });
                });
            }

            dbg!(&first_o);

            dbg!(&second_o);
            // .ok()
            // .expect("No first O+ link found");
          }
          _ => (),
        }
      }

      // "MV" links can be used to link to receipients of an action.
      if aw.has_disjunct(LinkTypes::MV, ConnectorPointing::Right) {
        if let Some(mv_right) = part
          .links
          .find_next_word_with_link(&aw, LinkTypes::MV, ConnectorPointing::Left)
        {
          if mv_right.has_disjunct(LinkTypes::J, ConnectorPointing::Right) {
            // get recipient
            // For now this is considering a J link to be a recipient, but this could be true for a subset of words, e.g. "on", "for", "over".
            // Also, this might need also refer to a location (e.g. "on the table").
            if let Some(j_right) = part
              .links
              .find_next_word_with_link(&mv_right, LinkTypes::J, ConnectorPointing::Left)
            {
              let mut target_symbols = vec![];

              collect_j_target_words(&mut target_symbols, &j_right, &part, parse_state);

              dbg!(&target_symbols);

              for symbol in target_symbols {
                action
                  .properties
                  .push(ActionProperties::Recipient {
                    recipient: symbol.to_owned(),
                  });
              }
            }
          }
        }
      }
    }
  }

  Ok(action_connected_sentence)
}

// gets all words that are targets of a "J" link
// e.g. "Mary", "Mary and Steve", "Mary, Steve and John"
pub fn collect_j_target_words<'a>(
  target_symbols: &mut Vec<String>,
  word: &Word,
  part: &'a SentenceParts,
  parse_state: &mut ParseState,
) {
  for symbol in parse_state.get_symbols_by_position(word.position) {
    target_symbols.push(symbol.to_owned());
  }

  // if word_symbols.len() > 0 {
  //   target_symbols.extend(word_symbols);
  // }
  // let word_symbols = parse_state.get_symbols_by_position(word.position);

  // if word_symbols.len() > 0 {
  //   target_symbols.extend(word_symbols);
  // }

  if word.has_disjunct_with_prescript(LinkTypes::SJ, ConnectorPointing::Right, FreeWordOrder::Head)
  {
    // println!("1");
    if let Some(next_word) = part
      .links
      .find_next_word_with_disjunct(
        &word,
        LinkTypes::SJ,
        ConnectorPointing::Left,
        FreeWordOrder::Dependent,
      )
    {
      // println!("2");
      collect_j_target_words(target_symbols, &next_word, part, parse_state);
    }
  }

  if word.has_disjunct_with_prescript(LinkTypes::SJ, ConnectorPointing::Left, FreeWordOrder::Head) {
    // println!("3");
    if let Some(prev_word) = part
      .links
      .find_prev_word_with_disjunct(
        &word,
        LinkTypes::SJ,
        ConnectorPointing::Right,
        FreeWordOrder::Dependent,
      )
    {
      // println!("4");
      collect_j_target_words(target_symbols, &prev_word, part, parse_state);
    }
  }
}
