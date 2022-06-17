use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;

pub type Tree = HashMap<String, TreeNode>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNode {
  pub word: String,
  pub branches: Tree,
}

pub fn init_wordnet_noun_objects(data_path: &str) -> Result<()> {
  let path = format!("{}/wordnet_noun_objects.json", data_path);
  let noun_objects = File::open(&path)?;
  let reader = BufReader::new(noun_objects);
  let wordnet_noun_objects = serde_json::from_reader(reader)?;

  WORDNET_NOUN_OBJECTS.set(wordnet_noun_objects).expect("Unable to set WORDNET_NOUN_OBJECTS");

  Ok(())
}

pub static WORDNET_NOUN_OBJECTS: OnceCell<Tree> = OnceCell::new();