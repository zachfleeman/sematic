use anyhow::Result;

use crate::nlp::sentence_parts::SentenceParts;
use crate::parse::link_parse::parse_with_links;
// use crate::parse::token_parse::parse_with_tokens;
use crate::sema::sema_sentence::SemaSentence;
use crate::services::sema_ai::get_ml_generated_sentence;

pub async fn process_parts(parts: Vec<SentenceParts>) -> Result<Vec<SemaSentence>> {
  let mut sema_sentences = Vec::new();

  for part in parts {
    let sema_sentence = process_part(part).await?;
    sema_sentences.push(sema_sentence);
  }

  Ok(sema_sentences)
}

pub async fn process_part(part: SentenceParts) -> Result<SemaSentence> {
  // First attempt is to use the token parser.
  // if let Some(s) = parse_with_tokens(part.clone())? {
  //   println!("parsed with tokens");
  //   dbg!(&s);
  //   return Ok(s);
  // }

  // Second attempt is to use the link parser.
  if let Some(s) = parse_with_links(part.clone())? {
    println!("parsed with links");
    return Ok(s);
  }

  // ultimate fallback is ML
  let ml_gen_sentences = get_ml_generated_sentence(vec![part
    .original_sentence
    .clone()])
  .await?;
  // assuming only one sentence is returned via this manner.
  let b = ml_gen_sentences
    .into_iter()
    .next()
    .ok_or("no sentence returned")
    .map_err(|e| anyhow::anyhow!(e))?;

  // Ok(sema_sentence)
  Ok(b.1.json)
}
