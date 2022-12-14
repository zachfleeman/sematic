use anyhow::Result;
use std::collections::HashMap;

use link_parser_rust_bindings::{
  lp::{disjunct::ConnectorPointing, link_types::LinkTypes, word::Word},
  pos::POS,
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    entity::{Entity, EntityProperties, Quantities},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use super::link_parse::ParseState;
// use crate::parse::link_parse::parse_temporal::TIME_NOUNS;
use crate::parse::numbers::*;
use crate::wordnet::wordnet_noun_objects::{Tree, WORDNET_NOUN_OBJECTS};

pub fn build_noun_phrases(
  words: &mut Vec<Word>,
  part: &SentenceParts,
  branch: &Tree,
  noun_phrase: &mut Vec<Word>,
  noun_phrases: &mut Vec<Vec<Word>>,
) -> Result<()> {
  if let Some(word) = words.pop() {
    // println!(
    //   "recurse: {:?}: morpho: {}",
    //   word.get_cleaned_word(),
    //   word.morpho_guessed
    // );

    let pos = word
      .pos
      .unwrap_or(POS::Undefined);

    if noun_phrase.len() == 0 {
      if (matches!(
        pos,
        POS::Noun | POS::NounUncountable | POS::PluralCountNoun | POS::SingularMassNoun
      ) | word.morpho_guessed)
        && !word.has_raw_disjunct("J+")
        && !word.word_is_i()
      {
        let lemma = part
          .get_word_lemma(&word)
          .to_lowercase();
        // println!("lemma 1: {:?}, word.position: {:?}", &lemma, word.position);

        if branch.contains_key(&lemma){
          let tree_node = branch
            .get(&lemma)
            .unwrap();

          // println!("noun_phrase.push 1");
          noun_phrase.push(word);
          build_noun_phrases(words, part, &tree_node.branches, noun_phrase, noun_phrases)?;
        } else if word.all_upper {
          // got an abbreviation.
          let empty_hash_map = HashMap::new();

          noun_phrase.push(word);
          build_noun_phrases(words, part, &empty_hash_map, noun_phrase, noun_phrases)?;
        } else {
          // println!("else 1");
          if noun_phrase.len() > 0 {
            noun_phrase.reverse();
            noun_phrases.push(noun_phrase.clone());
            noun_phrase.clear();
          }
        }
      } else {
        // println!("else 2");
        if noun_phrase.len() > 0 {
          noun_phrase.reverse();
          noun_phrases.push(noun_phrase.clone());
          noun_phrase.clear();
        }
      }
    } else {
      // already got a first noun, and recursing to see if it's a whole phrase.
      // dbg!(&word);
      // let token = part.get_word_token(&word);
      // dbg!(&token);
      let lemma = part
        .get_word_lemma(&word)
        .to_lowercase();
      // println!("lemma 2: {:?}, word.position: {:?}", &lemma, word.position);

      if branch.contains_key(&lemma) {
        let tree_node = branch
          .get(&lemma)
          .unwrap();

        // println!("noun_phrase.push 2");
        noun_phrase.push(word);
        build_noun_phrases(words, part, &tree_node.branches, noun_phrase, noun_phrases)?;
      } else if word.has_disjunct(LinkTypes::AN, ConnectorPointing::Right) {
        // if word branches aren't in dictionary, might still be a noun phrase.
        // Look to see if word is a noun with an "AN" disjunct.
        // "AN" connects noun-modifiers to following nouns: "The TAX PROPOSAL was rejected".
        // println!("noun_phrase.push 3");
        noun_phrase.push(word);
        let empty_hash_map = HashMap::new();
        build_noun_phrases(words, part, &empty_hash_map, noun_phrase, noun_phrases)?;
      } else {
        if noun_phrase.len() > 0 {
          // println!("else 3");
          noun_phrase.reverse();
          noun_phrases.push(noun_phrase.clone());
          noun_phrase.clear();
        }
      }
    }
  } else {
    // println!("else 4");
    if noun_phrase.len() > 0 {
      noun_phrase.reverse();
      noun_phrases.push(noun_phrase.clone());
      noun_phrase.clear();
    }
  }

  Ok(())
}

pub fn get_entity_key(noun_phrase: &Vec<Word>, part: &SentenceParts) -> Result<String> {
  let entity_key = match noun_phrase.len() {
    1 => {
      let word = noun_phrase
        .get(0)
        .unwrap();

      part
        .get_word_lemma(word)
        .to_lowercase()
    }
    _ => noun_phrase
      .iter()
      .map(|word| {
        part
          .get_word_lemma(word)
          .to_lowercase()
      })
      .collect::<Vec<String>>()
      .join("_"),
  };

  Ok(entity_key)
}

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

  let mut words = part
    .links
    .words
    .clone();

  let noun_objects = WORDNET_NOUN_OBJECTS
    .get()
    .unwrap();

  let mut current_noun_phrase = vec![];
  let mut noun_phrase_arrays = vec![];

  while words.len() > 0 {
    build_noun_phrases(
      &mut words,
      part,
      noun_objects,
      &mut current_noun_phrase,
      &mut noun_phrase_arrays,
    )?;
  }

  // println!("noun_phrase_arrays: {:?}", noun_phrase_arrays.len());
  // dbg!(&noun_phrase_arrays);

  for noun_phrase in noun_phrase_arrays {
    let entity_key = get_entity_key(&noun_phrase, part)?;
    // println!("entity_key: {}", &entity_key);

    let mut noun_mods: Vec<EntityProperties> = vec![];
    let mut entity = Entity::new(entity_key, symbol);

    if let Some(first_word) = noun_phrase.first() {
      if let Some(prev_word) = part
        .links
        .get_prev_word(&first_word)
      {
        get_noun_modifiers(&mut noun_mods, vec![&first_word], prev_word, part);

        // if noun is plural and doesn't already have an aplicable modifier (Count, Multiple), add one.
        if let Some(last_word) = noun_phrase.last() {
          let has_plural_mod = noun_mods
            .iter()
            .any(|prop| match prop {
              EntityProperties::Modifier {
                modifier_type: _,
                modifier: _,
                amplifiers: _,
              } => false,
              EntityProperties::Count { count: _ } => true,
              EntityProperties::Quantity { quantity: _ } => true,
              EntityProperties::Occurance { occurs: _ } => false,
              EntityProperties::Attribute { attribute: _ } => false,
            });

          // println!("has_plural_mod: {}", has_plural_mod);

          if !has_plural_mod {
            // see if the last word is plural, and if so, add a "plural" modifier.
            if let Some(plurality) = part.get_word_plurality(&last_word) {
              if plurality == "plural" {
                noun_mods.push(EntityProperties::Quantity {
                  quantity: Quantities::Multiple,
                });
              }
            }
          }
        }
      }

      entity
        .properties
        .extend(noun_mods);

      let word_positions = noun_phrase
        .iter()
        .map(|word| word.position)
        .collect::<Vec<usize>>();

      parse_state.add_symbol(&entity.symbol, word_positions);

      output_sentence
        .entities
        .push(entity);
    }
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

  // checking to see if the word before the current word is "the"
  if word.has_disjunct(LinkTypes::AL, ConnectorPointing::Left)
    && word.position
      == noun
        .first()
        .unwrap()
        .position
        - 1
    && word.get_cleaned_word() == "the"
  {
    println!("the");
    if let Some(al_word) = part
      .links
      .find_prev_word_with_link(word, LinkTypes::AL, ConnectorPointing::Right)
    {
      match al_word
        .get_cleaned_word()
        .as_str()
      {
        "all" => {
          println!("all");
          entity_mods.push(EntityProperties::Quantity {
            quantity: Quantities::All,
          });
        }
        "both" => {
          println!("both");
          entity_mods.push(EntityProperties::Count { count: 2. });
        }
        _ => {
          println!("other");
        }
      }
    }
  }

  /*t
  LEFT-WALL   hWg+ RW+
  find.v      Wg- O+
  all.a       Dm+
  books.n     Dmc- Op-
  RIGHT-WALL  RW-
  */
  if word.has_raw_disjunct("Dm+") {
    //
  }

  if word.has_raw_disjunct("Dmc+") {
    match word
      .get_cleaned_word()
      .as_str()
    {
      "all" => {
        entity_mods.push(EntityProperties::Quantity {
          quantity: Quantities::All,
        });
      }
      "both" => {
        entity_mods.push(EntityProperties::Count { count: 2. });
      }
      _ => {
        println!("!!!")
      }
    }
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
  if word.has_pos(POS::Adjective)
    && !word.has_raw_disjunct("DTi+")
    && !word.has_raw_disjunct("ALx+")
  /* Already added with "the" check above */
  {
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
