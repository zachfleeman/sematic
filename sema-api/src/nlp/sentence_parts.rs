use super::duck::DuckPart;
use super::nlp_rule::NLPRule;
use super::{chunk::Chunk, duck::Duck};
use anyhow::Result;
use link_parser_rust_bindings::lp::{sentence::Sentence as LPSentence, word::Word as LPWord};
use nlprule::types::owned::Token;
use urlencoding::decode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentenceEncodings {
  #[serde(rename = "none")]
  None,

  #[serde(rename = "url")]
  URL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceText {
  pub original_text: String,

  pub decoded_text: String,

  pub repaired_text: Option<String>,
}

impl SentenceText {
  pub fn new(original_text: String, encoding: SentenceEncodings, repair: bool) -> Result<SentenceText> {
    let decoded_text = match encoding {
      SentenceEncodings::None => original_text.clone(),
      SentenceEncodings::URL => decode(&original_text)?.into_owned(),
    };

    let repaired_text = if repair {
      Some(NLPRule::correct(&decoded_text)?)
    } else {
      None
    };

    Ok(SentenceText {
      original_text,
      decoded_text,
      repaired_text,
    })
  }

  pub fn text(&self) -> &str {
    match &self.repaired_text {
      Some(text) => &text,
      None => &self.decoded_text,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceParts {
  pub original_sentence: String,

  pub corrected_sentence: String,

  pub lemmatized_sentence: String,

  pub tokens: Vec<Token>,

  pub links: LPSentence,

  pub chunks: Vec<Chunk>,

  pub duck: Duck,
}

impl SentenceParts {
  pub fn from_text(sentence_text: &SentenceText) -> Result<SentenceParts> {
    // let decoded_og_sentence = match parts_config.encoding {
    //     SentenceEncodings::None => parts_config.original_sentence.to_string(),
    //     SentenceEncodings::URL => decode(&parts_config.original_sentence)?.into_owned(),
    // };
    
    // dbg!(&decoded_og_sentence);
    
    // let corrected_sentence = if parts_config.repair {
    //   NLPRule::correct(decoded_og_sentence)?
    // } else {
    //   decoded_og_sentence
    // };

    // dbg!(&corrected_sentence);

    let nlp_sentence = NLPRule::tokenize(sentence_text.text())?;

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

    println!("a");
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

    println!("b");

    Ok(SentenceParts {
      original_sentence: sentence_text.decoded_text.clone(),
      corrected_sentence: sentence_text.text().to_owned(),
      lemmatized_sentence,
      tokens,
      links: LPSentence {
        ..Default::default()
      },
      chunks,
      duck: Duck::default(),
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

  pub fn get_word_token(&self, word: &LPWord) -> Option<&Token> {
    if word.position == 0 {
      return None;
    }

    let start = word
      .chars
      .start()
      .to_owned() as usize;

    self
      .tokens
      .iter()
      .find(|t| {
        t.span
          .char()
          .contains(&start)
      })
  }

  pub fn get_word_tokens(&self, word: &LPWord) -> Vec<&Token> {
    self
      .tokens
      .iter()
      .filter(|t| {
        let start = t.span.char().start as u64;
        let end = t.span.char().end as u64;
        word
          .chars
          .contains(&start)
          && word
            .chars
            .contains(&end)
      })
      .collect::<Vec<&Token>>()
  }

  pub fn get_word_lemma(&self, word: &LPWord) -> String {
    if let Some(token) = self.get_word_token(word) {
      token.word.tags[0]
        .lemma
        .clone()
        .as_ref()
        .to_string()
    } else {
      word.get_cleaned_word()
    }
  }

  pub fn get_word_plurality(&self, word: &LPWord) -> Option<String> {
    if let Some(token) = self.get_word_token(word) {
      if let Some(chunck_str) = token.chunks.get(0) {
        let plurality_option = chunck_str
          .split("-")
          .last();

        if let Some(plurality) = plurality_option {
          match plurality {
            "singular" => Some("singular".to_string()),
            "plural" => Some("plural".to_string()),
            _ => None,
          }
        } else {
          None
        }
      } else {
        None
      }
    } else {
      None
    }
  }

  pub fn get_word_ducklings(&self, word: &LPWord) -> Vec<&DuckPart> {
    if let Some(token) = self.get_word_token(word) {
      self
        .duck
        .parts
        .iter()
        .filter(|p| {
          p.chars.contains(
            &token
              .span
              .char()
              .start,
          )
        })
        .collect::<Vec<&DuckPart>>()
    } else {
      vec![]
    }
  }

  pub fn get_duck_words(&self, duck: &DuckPart) -> Vec<&LPWord> {
    self
      .links
      .words
      .iter()
      .filter(|w| {
        // let start = w.chars.start as usize;
        let start = w
          .chars
          .start()
          .to_owned() as usize;
        duck
          .chars
          .contains(&start)
      })
      .collect::<Vec<&LPWord>>()
  }

  pub fn get_duck_word_positions(&self, duck: &DuckPart) -> Vec<usize> {
    self
      .get_duck_words(duck)
      .iter()
      .map(|w| w.position)
      .collect::<Vec<usize>>()
  }
}
