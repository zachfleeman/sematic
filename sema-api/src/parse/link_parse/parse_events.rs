use anyhow::Result;
use link_parser_rust_bindings::lp::{disjunct::ConnectorPointing, link_types::LinkTypes};

// use link_parser_rust_bindings::{
//   lp::{disjunct::ConnectorPointing, link_types::LinkTypes, word::Word},
//   pos::POS,
// };

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

  // Starting simple.

  for word in part
    .links
    .words
    .iter()
  {
    // using the ON (on) linktype
    if word.has_disjunct(LinkTypes::ON, ConnectorPointing::Right) {
      let mut event = Event::new("event".to_string(), symbol);
      // find the word that the ON is pointing to
      let on_word = part
        .links
        .words
        .iter()
        .find(|w| w.has_disjunct(LinkTypes::ON, ConnectorPointing::Left));

      if let Some(w) = on_word {
        let symbols = parse_state.get_symbols_by_position(w.position);
        println!("symbols: {:?}", symbols);

        for symbol in symbols {
          event
            .properties
            .push(EventProperties::Occurance { occurs: symbol });
        }
      }

      if word.has_disjunct(LinkTypes::MV, ConnectorPointing::Left) {
        // get the verb (and action) that is connected to the MV
        let a = part
          .links
          .get_prev_words(word)
          .into_iter()
          .rev()
          .find(|w| w.has_disjunct(LinkTypes::MV, ConnectorPointing::Right));

        if let Some(w) = a {
          let symbols = parse_state.get_symbols_by_position(w.position);
          println!("action symbols: {:?}", symbols);

          for symbol in symbols {
            event
              .properties
              .push(EventProperties::Action { action: symbol });
          }
        }
      }

      // handle cases where the "on" is before the verb, e.g : "on March 20th, create a new folder"
      //

      if word.has_raw_disjunct("dCOa+") {
        // get the verb (and action) that is connected to the MV
        let a = part
          .links
          .get_next_words(word)
          .into_iter()
          .find(|w| w.has_raw_disjunct("hCO-"));

        if let Some(w) = a {
          let symbols = parse_state.get_symbols_by_position(w.position);
          println!("action symbols: {:?}", symbols);

          for symbol in symbols {
            event
              .properties
              .push(EventProperties::Action { action: symbol });
          }
        }
      }

      output_sentence
        .events
        .push(event);
    }
  }

  // Past tense verbs require an event.
  // let _past_tense_verbs = part
  //   .links
  //   .get_past_tense_verbs();

  /*
  First off need to create an event for actions(verbs) that are in the past tense.
  Also, maybe the future tense as well?
   */
  Ok(output_sentence)
}
