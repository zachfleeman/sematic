// https://www.abisource.com/projects/link-grammar/dict/section-O.html

use crate::lp::links::Plurality;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct O {
  pub plurality: Plurality,
}
