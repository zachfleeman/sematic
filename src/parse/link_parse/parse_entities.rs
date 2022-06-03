use anyhow::Result;

use link_parser_rust_bindings::{
  lp::{disjunct::ConnectorPointing, link_types::LinkTypes, word::Word},
  pos::POS,
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

  // Assuming single word nouns for now.
  let known_nouns = part
    .links
    .get_known_nouns();

  for noun in known_nouns.iter() {
    let mut entity = Entity::new(noun.word.clone(), symbol);

    let mut noun_mods: Vec<EntityProperties> = vec![];

    if let Some(prev_word) = part
      .links
      .get_prev_word(noun)
    {
      get_noun_modifiers(&mut noun_mods, vec![noun], prev_word, part);
    }

    entity
      .properties
      .extend(noun_mods);

    // if noun.has_disjunct(LinkTypes::A, ConnectorPointing::Left) {
    //   let mut entity_properties = vec![];
    //   for w in part.links.words[..noun.position]
    //     .iter()
    //     .rev()
    //   {
    //     // Stop if there are more (@)A- links
    //     if w.has_disjunct(LinkTypes::A, ConnectorPointing::Left) {
    //       break;
    //     }

    //     if w.has_raw_disjunct("dAJra-") && w.has_pos(POS::Adjective) {
    //       entity_properties.push(EntityProperties::Modifier {
    //         modifier_type: w.get_cleaned_word(),
    //         modifier: None,
    //         amplifiers: vec![],
    //       });
    //     }

    //     if w.has_disjunct(LinkTypes::A, ConnectorPointing::Right) {
    //       entity_properties.push(EntityProperties::Modifier {
    //         modifier_type: w.get_cleaned_word(),
    //         modifier: None,
    //         amplifiers: vec![],
    //       });
    //     }
    //   }

    //   entity.properties = entity_properties;
    // }

    parse_state.add_symbol(&entity.symbol, vec![noun.position]);

    repaired_sentence
      .entities
      .push(entity);
  }

  Ok(repaired_sentence)
}

pub fn get_noun_modifiers(
  entity_mods: &mut Vec<EntityProperties>,
  noun: Vec<&Word>,
  word: &Word,
  part: &SentenceParts,
) {
  // Stop at other A- links
  // TODO: There probably is better keys to trigger ending the recursion.
  if word.has_disjunct(LinkTypes::A, ConnectorPointing::Left) {
    return;
  }

  if word.has_pos(POS::Adjective) {
    let mut amplifiers = vec![];

    if let Some(prev_word) = part
      .links
      .get_prev_word(word)
    {
      if prev_word.has_disjunct(LinkTypes::EA, ConnectorPointing::Right) {
        amplifiers.push(prev_word.get_cleaned_word());
      }
    }

    let mut mod_prop = EntityProperties::Modifier {
      modifier_type: word.get_cleaned_word(),
      modifier: None,
      amplifiers,
    };

    entity_mods.push(mod_prop);
  }

  if word.position > 0 {
    if let Some(prev_word) = part
      .links
      .get_prev_word(word)
    {
      get_noun_modifiers(entity_mods, noun, prev_word, part);
    }
  }
}
