use anyhow::Result;
use link_parser_rust_bindings::lp::{
  disjunct::ConnectorPointing,
  link_types::LinkTypes,
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    entity::{Entity, EntityProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;

pub fn parse_entities(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  repaired_sentence
    .entities
    .clear();

  let known_nouns = part
    .links
    .get_known_nouns();

  for noun in known_nouns.iter() {
    let mut entity = Entity::new(noun.word.clone(), symbol);

    // let connected_words = part
    //   .links
    //   .get_connected_words(noun)?;
    // dbg!(&connected_words);

    // for cw in connected_words.iter() {
    //   // TODO: Need to check if the noun is in the wordnet as multiple words.
    //   // Maybe some of this work should happen when getting the nouns?
    //   // For now just assuming all nouns are single words.
    //   if noun.has_disjunct(LinkTypes::A, ConnectorPointing::Left)
    //     && cw.has_disjunct(LinkTypes::A, ConnectorPointing::Right)
    //   {
    //     let mod_type = cw.word.clone();
    //     let mut amps = vec![];

    //     if cw.has_disjunct(LinkTypes::EA, ConnectorPointing::Left) {
    //       let amp_words = part
    //         .links
    //         .get_connected_words(cw)?;
    //       dbg!(&amp_words);

    //       for amp_word in amp_words.iter() {
    //         if amp_word.has_disjunct(LinkTypes::EA, ConnectorPointing::Right) {
    //           amps.push(
    //             amp_word
    //               .word
    //               .clone(),
    //           );
    //         }
    //       }
    //     };

    //     entity
    //       .properties
    //       .push(EntityProperties::Modifier {
    //         modifier_type: mod_type,
    //         modifier: None,
    //         amplifiers: amps,
    //       });
    //   }
    // }

    parse_state.add_symbol(&entity.symbol, vec![noun.position]);

    repaired_sentence
      .entities
      .push(entity);
  }

  Ok(repaired_sentence)
}
