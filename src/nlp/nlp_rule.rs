use anyhow::Result;
use std::fmt;

use once_cell::sync::OnceCell;

use nlprule::{rules_filename, tokenizer_filename, Rules, Tokenizer, types::Sentence};

pub struct NLPRule {
  tokenizer: Tokenizer,
  rules: Rules,
}

impl NLPRule {
  pub fn new() -> Result<Self> {
    let mut tokenizer_bytes: &'static [u8] =
      include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en")));

    let mut rules_bytes: &'static [u8] =
      include_bytes!(concat!(env!("OUT_DIR"), "/", rules_filename!("en")));

    let tokenizer =
      Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");

    let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is valid");

    Ok(NLPRule { tokenizer, rules })
  }

  pub fn init() {
    let nlp_rule = NLPRule::new().expect("Unable to create NLPRule instance");
    NLP_RULE.set(nlp_rule).expect("Unable to set NLPRULE");
  }

  pub fn correct(sentence: String) -> Result<String> {
    let nlp_rule = NLP_RULE.get().expect("NLP_RULE is not initialized");

    let corrected = nlp_rule.rules.correct(&sentence, &nlp_rule.tokenizer);

    Ok(corrected)
  }

  pub fn tokenize(sentence: &str) -> Result<Sentence> {
    let nlp_rule = NLP_RULE.get().expect("NLP_RULE is not initialized");

    let tokens = nlp_rule.tokenizer.pipe(sentence).next().unwrap();

    Ok(tokens)
  }
}

impl fmt::Debug for NLPRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "NLPRule")
  }
}

pub static NLP_RULE: OnceCell<NLPRule> = OnceCell::new();

// pub fn init_nlp_rule_tokenizer() -> Result<()> {
//   NLP_RULE_TOKENIZER
//     .set(NLPRuleTokenizer::new()?)
//     .map_err(|e| e.into())
// }
