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
use crate::parse::link_parse::parse_temporal::TIME_NOUNS;
use crate::parse::numbers::*;

pub fn parse_entities(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut output_sentence = sema_sentence.clone();

  output_sentence
    .entities
    .clear();

  // Assuming single word nouns for now.
  let known_nouns = part
    .links
    .get_known_nouns()
    .into_iter()
    .filter(|w| !TIME_NOUNS.contains(&w.get_cleaned_word().as_str()))
    .collect::<Vec<_>>();

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

    parse_state.add_symbol(&entity.symbol, vec![noun.position]);

    output_sentence
      .entities
      .push(entity);
  }

  Ok(output_sentence)
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
    if !is_num_word(&word.get_cleaned_word()) {
      return;
    }
  }

  // might be other cases where count will be on the wrong noun. TODO: fix this.
  if word.has_raw_disjunct("Dmc-") {
    return;
  }

  if word.has_raw_disjunct("Dmcn+") {
    // got a number on our hands.
    // NOTE: a fuzzy case is when number should be hyphenated, but are not.
    // e.g. "thirty five", but should be "thirty-five"
    // Neet to still handle this case.
    if let Some(count) = construct_number(word, part) {
      entity_mods.push(EntityProperties::Count { count });

      return;
    }
  }

  // "DTi+" is used to link determiners with nouns
  if word.has_pos(POS::Adjective) && !word.has_raw_disjunct("DTi+") {
    let mut amplifiers = vec![];

    if let Some(prev_word) = part
      .links
      .get_prev_word(word)
    {
      if prev_word.has_disjunct(LinkTypes::EA, ConnectorPointing::Right) {
        amplifiers.push(prev_word.get_cleaned_word());
      }
    }

    let mod_prop = EntityProperties::Modifier {
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
