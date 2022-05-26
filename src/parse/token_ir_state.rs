use crate::nlp::sentence_parts::SentenceParts;
use crate::nlp::treebank_pos::TreebankPOS;
use crate::parse::ir::IR;
use nlprule::types::owned::Token;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenIRState {
  pub part: SentenceParts,
  pub current_index: usize,
  pub ir: Vec<IR>,
}

impl TokenIRState {
  pub fn new(part: SentenceParts) -> Self {
    TokenIRState {
      part,
      current_index: 0,
      ir: vec![],
    }
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
    println!(
      "current: {:?} tokens: {:?}",
      self.current_index,
      self
        .part
        .tokens
        .len()
    );
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
