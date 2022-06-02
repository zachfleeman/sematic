use super::ir::*;
use super::token_ir_state::TokenIRState;


use anyhow::Result;

use crate::nlp::sentence_parts::SentenceParts;
use crate::sema::{
  action::{Action, ActionProperties},
  agents::*,
  entity::Entity,
  sema_sentence::SemaSentence,
  symbol::Symbol,
};

pub fn parse_with_tokens(part: SentenceParts) -> Result<Option<SemaSentence>> {
  let token_ir_state: TokenIRState = TokenIRState::new(part.clone())?;

  let mut symbol = Symbol::new(0);

  let parse_attempt = match &token_ir_state.ir[..] {
    // "create"
    [IR::Action(action)] => {
      println!("{:?}", action);

      let action = Action::new(action.text.clone(), &mut symbol);

      let mut sentence = SemaSentence::new();
      sentence
        .actions
        .push(action);

      Some(sentence)
    }

    // "Mary"
    [IR::Person(ir_person)] => {
      let mut person = Person::new(&mut symbol);

      person
        .properties
        .push(PersonProperties::Name {
          name: ir_person
            .text
            .clone(),
        });

      let mut sentence = SemaSentence::new();
      sentence
        .agents
        .push(Agents::Person(person));

        Some(sentence)
    }

    // Mary Jane (first and last name)
    [IR::Person(ir_person_1), IR::Person(ir_person_2)] => {
      let mut person = Person::new(&mut symbol);

      person
        .properties
        .push(PersonProperties::FirstName {
          first_name: ir_person_1
            .text
            .clone(),
        });
      person
        .properties
        .push(PersonProperties::LastName {
          last_name: ir_person_2
            .text
            .clone(),
        });

      let mut sentence = SemaSentence::new();
      sentence
        .agents
        .push(Agents::Person(person));

        Some(sentence)
    }
    // "create folder"
    [IR::Action(action), IR::Entity(parse_entity)] => {
      println!("got a match");

      let action = Action::new(action.text.clone(), &mut symbol);

      let entity = Entity::new(
        parse_entity
          .text
          .clone(),
        &mut symbol,
      );

      let mut sentence = SemaSentence::new();

      sentence
        .actions
        .push(action);

      sentence
        .entities
        .push(entity);

      Some(sentence)
    }
    // "create a folder"
    [IR::Action(parse_action), IR::Delimiter(_delim), IR::Entity(parse_entity)] => {
      println!("got a match");

      let action = Action::new(
        parse_action
          .text
          .clone(),
        &mut symbol,
      );

      let entity = Entity::new(
        parse_entity
          .text
          .clone(),
        &mut symbol,
      );

      let mut sentence = SemaSentence::new();

      sentence
        .actions
        .push(action);

      sentence
        .entities
        .push(entity);

      Some(sentence)
    }

    // Buy Jane a house
    [IR::Action(ir_action), IR::Person(ir_person), IR::Delimiter(_delim_1), IR::Entity(ir_entity)] =>
    {
      println!("Buy Jane a house");

      let mut action = Action::new(
        ir_action
          .text
          .clone(),
        &mut symbol,
      );

      let mut person = Person::new(&mut symbol);
      person
        .properties
        .push(PersonProperties::Name {
          name: ir_person
            .text
            .clone(),
        });

      let entity = Entity::new(
        ir_entity
          .text
          .clone(),
        &mut symbol,
      );

      action
        .properties
        .push(ActionProperties::Patient {
          patient: entity
            .symbol
            .clone(),
        });

      let mut sentence = SemaSentence::new();

      sentence
        .actions
        .push(action);

      sentence
        .entities
        .push(entity);

      sentence
        .agents
        .push(Agents::Person(person));

      Some(sentence)
    }

    // create a picture for Mary
    // Note: Person needs to be capitalized for now.
    [IR::Action(ir_action), IR::Delimiter(_ir_delim_1), IR::Entity(ir_entity), IR::CoordinatingConjunction(_cc_1), IR::Person(ir_person)] =>
    {
      println!("got a match");

      let action = Action::new(
        ir_action
          .text
          .clone(),
        &mut symbol,
      );

      let entity = Entity::new(
        ir_entity
          .text
          .clone(),
        &mut symbol,
      );

      let mut person = Person::new(&mut symbol);

      person
        .properties
        .push(PersonProperties::Name {
          name: ir_person
            .text
            .clone(),
        });

      let mut sentence = SemaSentence::new();

      sentence
        .actions
        .push(action);

      sentence
        .entities
        .push(entity);

      sentence
        .agents
        .push(Agents::Person(person));

      Some(sentence)
    }

    _ => {
      None
      // let sentence = SemaSentence::new();

      // let verified_sema_sentence_json =
      //   verify_and_repair_sema_json(sentence, part).await?;

      // // dbg!(&verified_sema_sentence_json);

      // // b.1.json
      // verified_sema_sentence_json
    }
    // _ => {
    //   let a = get_ml_generated_sentence(vec![part
    //     .original_sentence
    //     .clone()])
    //   .await?;
    //   // assuming only one sentence is returned via this manner.
    //   let b = a
    //     .into_iter()
    //     .next()
    //     .ok_or("no sentence returned")
    //     .map_err(|e| anyhow::anyhow!(e))?;

    //   let sema_sentence_json = b.1.json.clone();

    //   let verified_sema_sentence_json =
    //     verify_and_repair_sema_json(sema_sentence_json.clone(), part).await?;

    //   dbg!(&verified_sema_sentence_json);

    //   // b.1.json
    //   verified_sema_sentence_json
    // }
  };

  Ok(parse_attempt)
}
