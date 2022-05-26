pub mod ir;
use crate::parse::ir::*;

pub mod token_ir_state;
use crate::parse::token_ir_state::TokenIRState;

use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, IResult};

use std::str::FromStr;

use nlprule::types::owned::Token;

use crate::nlp::sentence_parts::SentenceParts;
use crate::nlp::treebank_pos::TreebankPOS;
use crate::sema::action::Action;
use crate::sema::entity::Entity;
use crate::sema::sema_sentence::SemaSentence;
use crate::sema::symbol::Symbol;
use crate::services::sema_ai::get_ml_generated_sentence;
use crate::wordnet::wordnet_verbs::WordnetVerbs;

// pub type ParsedResult = IResult<Vec<Token>, Vec<Option<IR>>>;
// pub type TokenIRState = (Vec<Token>, Vec<Option<IR>>);

pub fn get_token_pos(token: &Token, index: usize) -> Result<TreebankPOS> {
  let raw_pos = token.word.tags[index]
    .pos
    .as_ref();

  TreebankPOS::from_str(raw_pos).map_err(|_| anyhow!("Could not parse pos: {}", raw_pos))
}

pub fn get_token_text(token: &Token) -> String {
  token
    .word
    .text
    .as_ref()
    .to_owned()
}

pub fn tokens_to_ir(input: TokenIRState) -> Result<TokenIRState> {
  // println!("parse tokens");
  let mut output = input.clone(); // I don't think I need to clone here.
  let token = output.get_current_token();
  let pos = get_token_pos(&token, 0)?;
  // println!("pos: {:?}", pos);

  match pos {
    TreebankPOS::CC => (),
    TreebankPOS::CD => (),
    TreebankPOS::DT => {
      output.push_parsed_token(IR::Delimiter(IRDelimiter {
        text: get_token_text(&token),
      }));
    }
    TreebankPOS::EX => (),
    TreebankPOS::FW => (),
    TreebankPOS::IN => (),
    TreebankPOS::JJ => (),
    TreebankPOS::JJR => (),
    TreebankPOS::JJS => (),
    TreebankPOS::LS => (),
    TreebankPOS::MD => (),
    TreebankPOS::NN => {
      output.push_parsed_token(IR::Entity(IREntity {
        token_index: output.current_index,
        text: get_token_text(&token),
        plural: false,
      }));
    }
    TreebankPOS::NNS => {
      output.push_parsed_token(IR::Entity(IREntity {
        token_index: output.current_index,
        text: get_token_text(&token),
        plural: true,
      }));
    }
    TreebankPOS::NNU => (),
    TreebankPOS::NNUN => {
      // case: "watch a movie"
      // NNUN should be an action if:
      // 1. This is the first token
      // 2. the second tag is "VB" (verb)
      if output.current_index == 0
        && token
          .word
          .tags
          .len()
          > 2
      {
        let second_pos = get_token_pos(&token, 1)?;

        if second_pos == TreebankPOS::VB {
          output.push_parsed_token(IR::Action(IRAction {
            token_index: output.current_index,
            text: get_token_text(&token),
          }));
        }
      }
    }
    TreebankPOS::NNP => (),
    TreebankPOS::NNPS => (),
    TreebankPOS::ORD => (),
    TreebankPOS::PCT => (),
    TreebankPOS::PDT => (),
    TreebankPOS::POS => (),
    TreebankPOS::PRP => (),
    TreebankPOS::PRP2 => (),
    TreebankPOS::RB => (),
    TreebankPOS::RBR => (),
    TreebankPOS::RBS => (),
    TreebankPOS::RBSENT => (),
    TreebankPOS::RP => (),
    TreebankPOS::SYM => (),
    TreebankPOS::TO => (),
    TreebankPOS::UH => (),
    TreebankPOS::VB => output.push_parsed_token(IR::Action(IRAction {
      token_index: output.current_index,
      text: get_token_text(&token),
    })),
    TreebankPOS::VBD => (),
    TreebankPOS::VBG => (),
    TreebankPOS::VBN => (),
    TreebankPOS::VBP => (),
    TreebankPOS::VBZ => (),
    TreebankPOS::WDT => (),
    TreebankPOS::WP => (),
    TreebankPOS::WP2 => (),
    TreebankPOS::WRB => (),
  }
  // dbg!(&output.ir);

  Ok(output)
}

pub async fn parse_sentence_part(part: SentenceParts) -> Result<SemaSentence> {
  let mut token_ir_state: TokenIRState = TokenIRState::new(part.clone());

  // NOTE: Should this be fully recursive? Without the while loop?
  while !token_ir_state.finished() {
    token_ir_state = tokens_to_ir(token_ir_state)?;
    token_ir_state.current_index += 1;
  }

  dbg!(&token_ir_state.ir);
  let mut symbol = Symbol::new(0);

  let sema_sentence = match &token_ir_state.ir[..] {
    // "create"
    [IR::Action(action)] => {
      println!("{:?}", action);

      let action = Action::new(action.text.clone(), &mut symbol);

      let mut sentence = SemaSentence::new();
      sentence
        .actions
        .push(action);

      sentence
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

      sentence
    }
    // "create a folder"
    [IR::Action(parse_action), IR::Delimiter(delim), IR::Entity(parse_entity)] => {
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

      sentence
    }

    [IR::Action(action), IR::Delimiter(delim_1), IR::Entity(entity1), IR::Delimiter(delim_2), IR::Entity(entity2)] => {
      SemaSentence::new()
    }
    _ => {
      let a = get_ml_generated_sentence(vec![part
        .original_sentence
        .clone()])
      .await?;
      // assuming only one sentence is returned via this manner.
      let b = a
        .into_iter()
        .next()
        .unwrap();

      b.1.json
    }
  };

  Ok(sema_sentence)
}
