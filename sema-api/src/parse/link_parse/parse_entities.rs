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
use crate::wordnet::wordnet_noun_objects::{Tree, WORDNET_NOUN_OBJECTS};

// pub fn recurse(words: Vec<Word>, part: &SentenceParts, accum: &mut Vec<String>) {
// }

// pub fn collect_nouns(words: Vec<Word>) -> Result<Vec<Vec<Word>>> {
//   let mut local_words = words.clone();
//   let mut collected_nouns = vec![];
//   let mut accum = vec![];

//   let noun_objects = WORDNET_NOUN_OBJECTS
//     .get()
//     .unwrap();

//   while local_words.len() > 0 {
//     if let Some(word) = local_words.pop() {
//       if let Some(pos) = word.pos {
//         if matches!(
//           pos,
//           POS::Noun | POS::NounUncountable | POS::PluralCountNoun | POS::SingularMassNoun
//         ) | word.morpho_guessed
//         {
//           if noun_objects.contains_key(&word.get_cleaned_word()) {
//             accum.push(word);
//           } else {
//             collected_nouns.push(accum);
//             accum = vec![];
//           }
//         } else {
//           collected_nouns.push(accum);
//           accum = vec![];
//         }
//       }
//     }
//   }

//   Ok(collected_nouns)
// }

pub fn recurse(
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
      if matches!(
        pos,
        POS::Noun | POS::NounUncountable | POS::PluralCountNoun | POS::SingularMassNoun
      ) | word.morpho_guessed
      {
        let lemma = part
          .get_word_lemma(&word)
          .to_lowercase();
        // println!("lemma 1: {:?}, word.position: {:?}", &lemma, word.position);

        if branch.contains_key(&lemma) {
          let tree_node = branch
            .get(&lemma)
            .unwrap();

          // println!("noun_phrase.push 1");
          noun_phrase.push(word);
          recurse(words, part, &tree_node.branches, noun_phrase, noun_phrases)?;
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
        recurse(words, part, &tree_node.branches, noun_phrase, noun_phrases)?;
      } else {
        // println!("else 3");
        if noun_phrase.len() > 0 {
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
    recurse(
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
    let entity_key = noun_phrase
      .iter()
      .map(|word| {
        word
          .get_cleaned_word()
          .to_lowercase()
      })
      .collect::<Vec<String>>()
      .join("_");

    if let Some(first_word) = noun_phrase.first() {
      let mut entity = Entity::new(entity_key, symbol);
  
      let mut noun_mods: Vec<EntityProperties> = vec![];
  
      if let Some(prev_word) = part
        .links
        .get_prev_word(&first_word)
      {
        get_noun_modifiers(&mut noun_mods, vec![&first_word], prev_word, part);
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

  // Assuming single word nouns for now.
  // let known_nouns = part
  //   .links
  //   .get_known_nouns()
  //   .into_iter()
  //   .filter(|w| {
  //     !TIME_NOUNS.contains(
  //       &w.get_cleaned_word()
  //         .as_str(),
  //     )
  //   })
  //   .collect::<Vec<_>>();

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
