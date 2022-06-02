use anyhow::Result;
use link_parser_rust_bindings::lp::disjunct::{ConnectorPointing, LinkTypes};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    action::Action,
    agents::{Agents, Ego},
    entity::{Entity, EntityProperties},
    sema_sentence::SemaSentence,
    symbol::Symbol,
  },
};

pub async fn verify_and_repair_sema_json(
  sema_sentence: SemaSentence,
  part: SentenceParts,
) -> Result<SemaSentence> {
  let mut symbol = Symbol::new(0);
  let mut repaired_sentence = sema_sentence.clone();

  if verify_actions(&sema_sentence, &part)? {
    // return Ok(());
    println!("actions verified");
  } else {
    println!("actions not verified");
    repaired_sentence = repair_actions(&sema_sentence, &part, &mut symbol)?;
  }

  let repaired_sentence = repair_agents(&repaired_sentence, &part, &mut symbol)?;

  let repaired_sentence = repair_entities(&repaired_sentence, &part, &mut symbol)?;

  Ok(repaired_sentence)
}

pub fn repair_agents(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  // "I" and "My" detection
  let ego_exists_in_sentence = part
    .links
    .words
    .iter()
    .any(|lp_word| {
      lp_word
        .word
        .to_lowercase()
        == "i"
        || lp_word
          .word
          .to_lowercase()
          == "my"
    });

  if ego_exists_in_sentence && !sema_sentence.has_ego_agent() {
    let agent = Agents::Ego(Ego::new(symbol));

    repaired_sentence
      .agents
      .push(agent);
  }

  Ok(repaired_sentence)
}

pub fn verify_actions(sema_sentence: &SemaSentence, part: &SentenceParts) -> Result<bool> {
  let verbs = part
    .links
    .get_verbs();
  // dbg!(&verbs);

  if sema_sentence
    .actions
    .len()
    != verbs.len()
  {
    return Ok(false);
  }

  for verb in verbs.iter() {
    let action = sema_sentence
      .actions
      .iter()
      .find(|action| action.action_type == verb.word);

    if action.is_none() {
      return Ok(false);
    }
  }

  Ok(true)
}

pub fn repair_actions(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  let verbs = part
    .links
    .get_verbs();
  // dbg!(&verbs);

  let actions = verbs
    .into_iter()
    .map(|verb| Action::new(verb.word.clone(), symbol))
    .collect::<Vec<Action>>();

  repaired_sentence
    .actions
    .clear();

  repaired_sentence.actions = actions;

  Ok(repaired_sentence)
}

pub fn repair_entities(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
) -> Result<SemaSentence> {
  let mut repaired_sentence = sema_sentence.clone();

  repaired_sentence.entities.clear();

  let known_nouns = part
    .links
    .get_known_nouns();

  // dbg!(&known_nouns);

  for noun in known_nouns.iter() {
    let mut entity = Entity::new(noun.word.clone(), symbol);

    let connected_words = part
      .links
      .get_connected_words(noun)?;
    // dbg!(&connected_words);

    for cw in connected_words.iter() {
      if noun.has_disjunct(LinkTypes::A, ConnectorPointing::Left)
        && cw.has_disjunct(LinkTypes::A, ConnectorPointing::Right)
      {
        let mod_type = cw.word.clone();
        let mut amps = vec![];

        if cw.has_disjunct(LinkTypes::EA, ConnectorPointing::Left) {
          let amp_words = part.links.get_connected_words(cw)?;
          dbg!(&amp_words);
  
          for amp_word in amp_words.iter() {
            if amp_word.has_disjunct(LinkTypes::EA, ConnectorPointing::Right) {
              amps.push(amp_word.word.clone());
            }
          }
        };


        entity
          .properties
          .push(EntityProperties::Modifier {
            modifier_type: mod_type,
            modifier: None,
            amplifiers: amps,
          });
      }
    }

    repaired_sentence
      .entities
      .push(entity);
  }

  // let entities = part
  //   .links
  //   .get_entities();
  // dbg!(&entities);

  // let entities = entities
  //   .into_iter()
  //   .map(|entity| {
  //     let mut agent = Person::new(symbol);

  //     agent
  //       .properties
  //       .push(PersonProperties::Name {
  //         name: entity.word.clone(),
  //       });

  //     Agents::Person(agent)
  //   })
  //   .collect::<Vec<Agents>>();

  // repaired_sentence
  //   .agents
  //   .clear();

  // repaired_sentence.agents = entities;

  Ok(repaired_sentence)
}
