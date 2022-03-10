use anyhow::Result;
use link_parser_rust_bindings::lp::{disjunct::LinkTypes, sentence::LPSentence, word::LPWord};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentenceTypes {
  Imperative,
  Interrogative,
  Exclamatory,
  Declarative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
  sentence_type: SentenceTypes,
}

impl Sentence {
  pub fn from_lp_sentence(lp_sentence: LPSentence) -> Sentence {
    // let's try to handle a few verbs
    let verbs = lp_sentence.get_verbs();

    // super hack
    for verb in verbs.into_iter() {
      if verb.word == "create" {
        println!("got a create");
        let stuff = handle_create_verb(lp_sentence.clone(), verb);
      }
    }

    Sentence {
      sentence_type: SentenceTypes::Imperative,
    }
  }
}

pub fn handle_create_verb(lp_sentence: LPSentence, verb: LPWord) -> Result<()> {
  // The "create" action

  // find out what we are creating
  let rest_of_sentence = lp_sentence.words[verb.position + 1..].to_vec();

  let a = rest_of_sentence
    .iter()
    .find(|w| {
      w.disjuncts.iter().any(|d| {
        match d.link_type {
          LinkTypes::O(_) => true,
          _ => false,
        }
      })
    });

  if let Some(left_pointing_o) = a {
    println!("{:?}", left_pointing_o);
  }

  Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionTypes {
  Create,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAction {
  action_type: ActionTypes, // should always be ActionTypes::Create
  handle: String,
  objects: Vec<String>,
  multiplier: i32, // would it be possible to have fractional multipliers?
}
