use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordnetNouns {
  words: HashSet<String>,
}

impl WordnetNouns {
  pub fn new() -> Result<Self> {
    let nouns = File::open("./data/wordnet_nouns.json")?;
    let reader = BufReader::new(nouns);
    let wordnet_nouns = serde_json::from_reader(reader)?;

    Ok(wordnet_nouns)
  }

  pub fn init() {
    let wordnet_nouns = WordnetNouns::new().expect("Unable to create WordnetNouns instance");
    WORDNET_NOUNS.set(wordnet_nouns).expect("Unable to set WORDNET_NOUNS");
  }

  pub fn contains(word: &str) -> bool {
    let wordnet_nouns = WORDNET_NOUNS.get().expect("WORDNET_NOUNS is not initialized");
    wordnet_nouns.words.contains(word)
  }
}

pub static WORDNET_NOUNS: OnceCell<WordnetNouns> = OnceCell::new();