use anyhow::Result;

use crate::{
  nlp::human_names::HumanNames,
  nlp::sentence_parts::SentenceParts,
  sema::{
    agents::{Agents, Ego, Subject, Person, PersonProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

use link_parser_rust_bindings::{
  lp::{disjunct::ConnectorPointing, word::Word as LPWord, link_types::LinkTypes},
  pos::POS,
};

use super::link_parse::ParseState;

pub static EGO_WORDS: [&str; 3] = ["i", "my", "me"];

pub static SUBJECT_QUESTION_WORDS: [&str; 1] = ["who"/* , "what", "which" */];

pub fn parse_agents(
  sema_sentence: &mut SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<()> {
  // "I" and "My" detection
  // Handling of basic people pronouns
  let ego_words = part
    .links
    .words
    .iter()
    .filter(|lp_word| {
      let w = lp_word.get_cleaned_word().to_lowercase();
      let w_ref = w.as_str();
      EGO_WORDS.contains(&w_ref)
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

  let subject_question_words = part
    .links
    .words
    .iter()
    .filter(|lp_word| {
      let w = lp_word.get_cleaned_word().to_lowercase();
      let w_ref = w.as_str();
      SUBJECT_QUESTION_WORDS.contains(&w_ref)
    })
    .collect::<Vec<_>>();

  if subject_question_words.len() > 0 {
    let subject_question_positions = subject_question_words
      .iter()
      .map(|lp_word| lp_word.position)
      .collect::<Vec<_>>();

    let subject = Subject::new(symbol);

    parse_state.add_symbol(&subject.symbol, subject_question_positions);

    let agent = Agents::Subject(subject);

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
