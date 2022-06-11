use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;

use crate::config::CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordnetVerbs {
  words: HashSet<String>,
}

impl WordnetVerbs {
  pub fn new() -> Result<Self> {
    println!("{:?}", std::env::current_dir());
    let mut pwd = std::env::current_dir()?;
    // pwd.push("data/wordnet_verbs.json");
    // let s = pwd.into_os_string().into_string();
    // dbg!(s);

    
    let config = CONFIG.get().unwrap();
    pwd.push(&config.data_path);
    pwd.push("wordnet_verbs.json");
    dbg!(&pwd);

    // let verbs_path = format!("{}/wordnet/verbs.json", config.data_path);
    // println!("{}", verbs_path);
    let verbs = File::open(pwd)?;
    // let verbs = File::open("../data/wordnet_verbs.json")?;
    let reader = BufReader::new(verbs);
    let wordnet_verbs = serde_json::from_reader(reader)?;

    Ok(wordnet_verbs)
  }

  pub fn init() {
    let wordnet_verbs = WordnetVerbs::new().expect("Unable to create WordnetVerbs instance");
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
