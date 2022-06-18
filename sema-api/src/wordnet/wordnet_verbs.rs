use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordnetVerbs {
  words: HashSet<String>,
}

impl WordnetVerbs {
  pub fn new(data_path: &str) -> Result<Self> {
    let path = format!("{}/wordnet_verbs.json", data_path);
    let verbs = File::open(&path)?;
    let reader = BufReader::new(verbs);
    let wordnet_verbs = serde_json::from_reader(reader)?;

    Ok(wordnet_verbs)
  }

  pub fn init(data_path: &str) {
    let wordnet_verbs = WordnetVerbs::new(data_path).expect("Unable to create WordnetVerbs instance");
    WORDNET_VERBS.set(wordnet_verbs).expect("Unable to set WORDNET_VERBS");
  }

  pub fn contains(word: &str) -> bool {
    let wordnet_verbs = WORDNET_VERBS
      .get()
      .expect("WORDNET_VERBS is not initialized");
    wordnet_verbs
      .words
      .contains(word)
  }
}

pub static WORDNET_VERBS: OnceCell<WordnetVerbs> = OnceCell::new();
