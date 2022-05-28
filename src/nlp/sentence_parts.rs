use super::chunk::Chunk;
use super::nlp_rule::NLPRule;
use anyhow::Result;
use link_parser_rust_bindings::{lp::sentence::LPSentence, LinkParserOptions, LinkParser};
use nlprule::types::owned::Token;

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceParts {
  pub original_sentence: String,

  pub corrected_sentence: String, // Not actually correct right now, due to how long it takes to run (~50ms)

  pub lemmatized_sentence: String,

  pub tokens: Vec<Token>,

  pub links: LPSentence,

  pub chunks: Vec<Chunk>,
}

impl SentenceParts {
  pub fn from_text(original_sentence: String, links: LPSentence) -> Result<SentenceParts> {
    let start = Instant::now();
    // let corrected_sentence = NLPRule::correct(original_sentence.clone())?;
    let corrected_sentence = original_sentence.clone();

    let nlp_sentence = NLPRule::tokenize(&corrected_sentence)?;
    let duration = start.elapsed();
    println!("nlp_sentence took: {:?}", duration.as_millis());

    let tokens = nlp_sentence
      .tokens()
      .iter()
      .map(|t| t.to_owned_token())
      .collect::<Vec<Token>>();

    let lemmatized_sentence = tokens
      .iter()
      .fold(String::new(), |acc, t| {
        let a = t.word.tags[0]
          .lemma
          .clone();
        let spacer = if t.has_space_before { " " } else { "" };
        acc + spacer + a.as_ref()
      });

    // Chunks!

    let mut current_chunk: Option<Chunk> = None;
    let mut chunks: Vec<Chunk> = vec![];

    for (i, token) in tokens
      .iter()
      .enumerate()
    {
      match token.chunks.len() {
        0 => {
          let phrase = tokens[i]
            .word
            .text
            .as_ref();
          let chunk = Chunk {
            pos: "unknown".to_string(),
            start: i,
            end: i,
            plural: None,
            phrase: phrase.to_owned(),
          };

          chunks.push(chunk);
        }
        1 => {
          let chunk_type = token.chunks[0].clone();
          let place = chunk_type
            .chars()
            .next()
            .unwrap();
          match place {
            'B' => {
              if let Some(mut chunk) = current_chunk.as_mut() {
                let phrase = tokens[chunk.start..=chunk.end]
                  .iter()
                  .map(|t| t.word.text.as_ref())
                  .collect::<Vec<&str>>()
                  .join(" ");

                chunk.phrase = phrase;

                chunks.push(chunk.clone());
              }

              current_chunk = Some(Chunk::from_chunk_type(chunk_type, i));
            }
            'I' => (),
            'E' => {
              if let Some(mut chunk) = current_chunk.as_mut() {
                chunk.end = i;

                let phrase = tokens[chunk.start..=chunk.end]
                  .iter()
                  .map(|t| t.word.text.as_ref())
                  .collect::<Vec<&str>>()
                  .join(" ");

                chunk.phrase = phrase;

                chunks.push(chunk.clone());

                current_chunk = None;
              }
            }
            'O' => (),
            _ => (),
          }
        }
        2 => {
          // Assuming for now that even though there are two "chunks" per token, they are of the same chunk.
          // Just the start and end of it.
          let chunk_type = token.chunks[0].clone();

          let mut chunk = Chunk::from_chunk_type(chunk_type, i);
          let phrase = tokens[i]
            .word
            .text
            .as_ref();

          chunk.phrase = phrase.to_owned();

          chunks.push(chunk);
        }
        _ => (),
      }
    }

    Ok(SentenceParts {
      original_sentence,
      corrected_sentence,
      lemmatized_sentence,
      tokens,
      links,
      chunks,
    })
  }

  pub fn get_chunk_tokens(&self, chunk_index: usize) -> Vec<Token> {
    let chunk = &self.chunks[chunk_index];
    let start = chunk.start;
    let end = chunk.end;

    self
      .tokens
      .clone()
      .into_iter()
      .skip(start)
      .take(end - start + 1)
      .collect::<Vec<Token>>()
  }
}
