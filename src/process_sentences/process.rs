use anyhow::Result;

use crate::nlp::sentence_parts::SentenceParts;
use crate::parse::parse_sentence_part;
use crate::sema::sema_sentence::SemaSentence;
use crate::sema::action::Action;
use crate::sema::entity::Entity;
use crate::sema::symbol::Symbol;

use crate::wordnet::wordnet_verbs::{WordnetVerbs};

pub async fn process_parts(parts: Vec<SentenceParts>) -> Result<Vec<SemaSentence>> {
  let mut sema_sentences = Vec::new();

  for part in parts {
    let sema_sentence = process_part(part).await?;
    sema_sentences.push(sema_sentence);
  }

  Ok(sema_sentences)
}

pub async fn process_part(part: SentenceParts) -> Result<SemaSentence> {
  let mut sema_sentence = SemaSentence::new();

  // if part.chunks.len() == 2 && is_two_chunk_sentence(&part) {
  //   println!("got a two chunker!");
  //   sema_sentence = create_two_chunk_sema_sentence(&part)?;
  // }
  let r = parse_sentence_part(part).await?;

  // Ok(sema_sentence)
  Ok(r)
}

// pub async fn process_single_verbs

pub fn is_two_chunk_sentence(part: &SentenceParts) -> bool {
  if part.chunks.len() > 2 {
    return false;
  };

  if part.chunks[0].pos != "VP" {
    return false;
  };

  let verb = part.chunks[0].phrase.clone();

  if !WordnetVerbs::contains(&verb) {
    return false;
  };

  return true;
}

pub fn create_two_chunk_sema_sentence(part: &SentenceParts) -> Result<SemaSentence> {
  let mut sema_sentence = SemaSentence::new();
  let mut symbol = Symbol::new(0);

  let verb = part.chunks[0].phrase.clone();

  let action = Action {
    action_type: verb,
    symbol: symbol.next_symbol(),
    properties: vec![],
  };

  sema_sentence.actions.push(action);

  let mut entity = chunk_to_entity(part, 1, &mut symbol)?;

  // entity.symbol = symbol.next_symbol();

  sema_sentence.entities.push(entity);

  Ok(sema_sentence)
}

pub fn chunk_to_entity(part: &SentenceParts, chunk_index: usize, symbol: &mut Symbol) -> Result<Entity> {
  // let chunk_tokens 
  let tokens = part.get_chunk_tokens(chunk_index);
  dbg!(&tokens);

  Ok(Entity::new("something".to_string(), symbol))
}