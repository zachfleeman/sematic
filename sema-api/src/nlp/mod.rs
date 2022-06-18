pub mod nlp_rule;
pub mod treebank_pos;
pub mod sentence_parts;
pub mod chunk;
pub mod human_names;

use self::nlp_rule::NLPRule;
use self::human_names::HumanNames;

pub fn init_nlp_cells(data_path: &str) {
  NLPRule::init();
  HumanNames::init(data_path);
}