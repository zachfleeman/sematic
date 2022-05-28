use crate::nlp::{
  human_names::HumanNames, sentence_parts::SentenceParts, treebank_pos::TreebankPOS,
};
use crate::parse::ir::*;
use crate::wordnet::wordnet_verbs::WordnetVerbs;
use anyhow::{anyhow, Result};
use nlprule::types::owned::Token;
use std::str::FromStr;

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
    TreebankPOS::CC => {
      output.push_parsed_token(IR::CoordinatingConjunction(IRCoordinatingConjunction {
        text: get_token_text(&token),
      }));
    }
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
        plurality: Plurality::Singular,
      }));
    }
    TreebankPOS::NNS => {
      output.push_parsed_token(IR::Entity(IREntity {
        token_index: output.current_index,
        text: get_token_text(&token),
        plurality: Plurality::Plural,
      }));
    }
    TreebankPOS::NNU => { // Nouns that are uncountable, like "everything", "admiration"
      output.push_parsed_token(IR::Entity(IREntity {
        token_index: output.current_index,
        text: get_token_text(&token),
        plurality: Plurality::Unknown
      }));
    },
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
    TreebankPOS::NNP => {
      // Proper Noun, singular. ex: "John", Denver, NORAD
      let text = get_token_text(&token);
      let is_human_name = HumanNames::contains(&text);

      if is_human_name {
        output.push_parsed_token(IR::Person(IRPerson {
          token_index: output.current_index,
          text: text,
        }));
      };
    }
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
    TreebankPOS::EMPTY => (),
    TreebankPOS::UNKNOWN => (),
  }

  Ok(output)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenIRState {
  pub part: SentenceParts,
  pub current_index: usize,
  pub ir: Vec<IR>,
}

impl TokenIRState {
  pub fn new(part: SentenceParts) -> Result<Self> {
    let mut token_ir_state = TokenIRState {
      part: part.clone(),
      current_index: 0,
      ir: vec![],
    };

    if part.tokens.len() == 1 && WordnetVerbs::contains(&get_token_text(&part.tokens[0])) {
      token_ir_state.push_parsed_token(IR::Action(IRAction {
        token_index: 0,
        text: get_token_text(&part.tokens[0]),
      }));
      return Ok(token_ir_state);
    } else {
      while !token_ir_state.finished() {
        token_ir_state = tokens_to_ir(token_ir_state)?;
        token_ir_state.current_index += 1;
      }
    }

    Ok(token_ir_state)
  }

  pub fn get_current_token(&self) -> Token {
    self.part.tokens[self.current_index].clone()
  }

  pub fn get_current_token_pos(&self) -> TreebankPOS {
    let token = self.get_current_token();
    let raw_pos = token.word.tags[0]
      .pos
      .as_ref();

    TreebankPOS::from_str(raw_pos).expect("Unable to get TreebankPOS for token pos")
  }

  pub fn finished(&self) -> bool {
    // println!(
    //   "current: {:?} tokens: {:?}",
    //   self.current_index,
    //   self
    //     .part
    //     .tokens
    //     .len()
    // );
    self.current_index + 1
      > self
        .part
        .tokens
        .len()
  }

  pub fn push_parsed_token(&mut self, parsed_token: IR) {
    self
      .ir
      .push(parsed_token);
  }
}
