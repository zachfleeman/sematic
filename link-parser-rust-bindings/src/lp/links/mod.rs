use serde_derive::{Deserialize, Serialize};

pub mod o;
pub mod p;

// pub use self::o::O;
// pub use self::p::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Plurality {
  Singular,
  Plural,
  Unknown,
}