use anyhow::Result;

use crate::{
  nlp::human_names::HumanNames,
  nlp::sentence_parts::SentenceParts,
  sema::{
    agents::{Agents, Ego, Person, PersonProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use link_parser_rust_bindings::{
  lp::{disjunct::ConnectorPointing, word::Word as LPWord, link_types::LinkTypes},
  pos::POS,
};

use super::link_parse::ParseState;

pub fn parse_agents(
  sema_sentence: &mut SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<()> {
  println!("Parse Agents");
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

  // human names
  let mut all_names: Vec<Vec<LPWord>> = vec![];
  let mut current_name = vec![];

  for word in part
    .links
    .words
    .iter()
  {
    let cleaned_word = word.get_cleaned_word();

    if let Some(pos) = word.pos {
      if matches!(
        pos,
        POS::GivenName | POS::GivenNameMasculine | POS::GivenNameFeminine
      ) {
        println!("Name! {}", word.word);
        current_name.push(word.clone());

        continue;
      }
    }

    if word.word_is_capitalized() {
      if word.morpho_guessed
        && HumanNames::contains(&cleaned_word)
        && !word.has_disjunct(LinkTypes::G, ConnectorPointing::Left) // G connects proper nouns words together
      {
        current_name.push(word.clone());
      }

      continue;
    } else {
      if current_name.len() > 0 {
        all_names.push(current_name);
        current_name = vec![];
      }
    }
  }

  for name_vec in all_names.iter() {
    let name_props = match name_vec.len() {
      1 => {
        let name = name_vec[0]
          .get_cleaned_word()
          .to_lowercase();
        vec![PersonProperties::Name { name }]
      }
      2 => {
        let first_name = name_vec[0]
          .get_cleaned_word()
          .to_lowercase();
        let last_name = name_vec[1]
          .get_cleaned_word()
          .to_lowercase();
        vec![
          PersonProperties::FirstName { first_name },
          PersonProperties::LastName { last_name },
        ]
      }
      _ => {
        let combined_name = name_vec
          .iter()
          .map(|w| {
            w.get_cleaned_word()
              .to_lowercase()
          })
          .collect::<Vec<String>>()
          .join("_");

        vec![PersonProperties::Name {
          name: combined_name,
        }]
      }
    };

    let mut person = Person::new(symbol);

    let name_positions = name_vec
      .iter()
      .map(|lp_word| lp_word.position)
      .collect::<Vec<_>>();

    parse_state.add_symbol(&person.symbol, name_positions);

    person
      .properties
      .extend(name_props);

    sema_sentence
      .agents
      .push(Agents::Person(person))
  }

  Ok(())
}
