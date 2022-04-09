use anyhow::Result;
use link_parser_rust_bindings::lp::{disjunct::LinkTypes, sentence::LPSentence, word::LPWord};
use serde_derive::{Deserialize, Serialize};

use super::{actions::{Actions}, Entity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentenceTypes {
  Imperative,
  Interrogative,
  Exclamatory,
  Declarative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
  pub sentence_type: SentenceTypes,

  pub entities: Vec<Entity>,

  pub actions: Vec<Actions>,
}

impl Sentence {
  pub fn new(sentence_type: SentenceTypes) -> Self {
    Sentence {
      sentence_type,
      entities: Vec::new(),
      actions: Vec::new(),
    }
  }

  // pub fn from_lp_sentence(lp_sentence: LPSentence) -> Sentence {
  //   // let's try to handle a few verbs
  //   let verbs = lp_sentence.get_verbs();

  //   // super hack
  //   for verb in verbs.into_iter() {
  //     // dbg!(verb);

  //     // if verb.word == "create" {
  //     //   println!("got a create");
  //       let stuff = handle_create_verb(lp_sentence.clone(), verb).expect("handle_create_verb: something went wrong");
  //     // }
  //   }

  //   Sentence {
  //     sentence_type: SentenceTypes::Imperative,
  //   }
  // }
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
    dbg!(&verb);
    dbg!(left_pointing_o);

    // What are the modifiers of left_pointing_o?
    println!("{} -> {}", verb.word, left_pointing_o.word);
  }

  Ok(())
}